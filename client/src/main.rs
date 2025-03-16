use clap::Parser;
use gps_tracker::RequestType;
use gps_tracker_client::UdpClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Parser, Serialize, Deserialize)]
pub struct Args {
    #[arg(short, long)]
    pub udp_client_address: String,
    #[arg(short, long)]
    pub udp_client_username: String,
    #[arg(short, long)]
    pub udp_client_password: String,
    #[arg(short, long)]
    pub udp_client_heartbeat_address: String,
    #[arg(short, long)]
    pub udp_client_data_path: String,
    #[arg(short, long)]
    pub udp_server_config_path: String,
    #[arg(short, long, default_value_t = 5)]
    pub coordinates_loop: u32,
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Args::parse();
    let client: Result<UdpClient, String> = UdpClient::new(
        args.udp_client_address.clone(),
        args.udp_client_username.clone(),
        args.udp_client_password.clone(),
        args.udp_server_config_path.clone(),
    )
    .await;
    let mut client: UdpClient = client.unwrap();
    let client_id: String = client.simulate(RequestType::Login, None, None).await?;

    let hb_client_id: String = client_id.clone();
    println!("Client Id: {}", client_id);
    let handler = tokio::spawn(async move {
        let client: Result<UdpClient, String> = UdpClient::new(
            args.udp_client_heartbeat_address.clone(),
            args.udp_client_username.clone(),
            args.udp_client_password.clone(),
            args.udp_server_config_path.clone(),
        )
        .await;
        let mut client: UdpClient = client.unwrap();

        loop {
            if let Err(error) = client
                .simulate(RequestType::HeartBeat, Some(hb_client_id.clone()), None)
                .await
            {
                panic!("{}", error);
            }
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        }
    });

    let path = args.udp_client_data_path;
    let coordinates_data = client.load_coordinates_data(path).await;
    let mut coordinates_data = coordinates_data.unwrap();
    for _ in 0..args.coordinates_loop {
        for item in &coordinates_data {
            let _ = client
                .simulate(
                    RequestType::Coordinates,
                    Some(client_id.clone()),
                    Some(item.to_owned()),
                )
                .await?;
        }
        coordinates_data.reverse();
    }
    let _ = tokio::time::timeout(std::time::Duration::from_secs(3), handler).await;
    let _ = client
        .simulate(RequestType::Logout, Some(client_id), None)
        .await;
    Ok(())
}
