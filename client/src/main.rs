use gps_tracker::config::Config;
use tokio::net::UdpSocket;
#[tokio::main]
async fn main() -> Result<(), String> {
    let config = Config::load(Some("../config.toml".to_string())).await?;
    let server = config.server;
    let host = server.host;

    match UdpSocket::bind("0.0.0.0:8086").await {
        Ok(socket) => match socket.connect(host).await {
            Ok(_) => {
                let mut buf = [0; 1024];
                if let Ok(size) = socket.send(&mut buf).await {
                    println!("Size: {}", size);
                }
                Ok(())
            }
            Err(error) => Err(error.to_string()),
        },
        Err(error) => Err(error.to_string()),
    }
}
