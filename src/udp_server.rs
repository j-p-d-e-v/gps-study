use crate::actions::{Coordinates, Heartbeat, Login, Logout};
use crate::config::Config;
use crate::{RequestPacket, RequestType};
use std::net::UdpSocket;

#[derive(Debug)]
pub struct UdpServer;

impl UdpServer {
    pub async fn launch() -> Result<(), String> {
        let config: Config = Config::load().await?;
        let server_config = config.server;
        println!("Server: {}", server_config.host);
        let socket = UdpSocket::bind(server_config.host).unwrap();
        loop {
            let mut buf = [0; 64];
            let (size, source_address) = socket.recv_from(&mut buf).unwrap();
            let filled = &mut buf[..size];
            match RequestPacket::parse(filled) {
                Ok(request_packet) => {
                    println!("Request Packet: {:?}", request_packet);
                    match request_packet.request_type {
                        RequestType::Login => {
                            let login_data = Login::parse(
                                request_packet.payload_length,
                                &request_packet.payload,
                            )
                            .await?;

                            println!("Login Data: {:?}", login_data);
                            let user_data = Login::authenticate(login_data).await?;
                            println!("User Data: {:?}", user_data);
                        }
                        RequestType::HeartBeat => {
                            let heartbeat_data = Heartbeat::parse(
                                source_address.to_string(),
                                request_packet.payload_length,
                                &request_packet.payload,
                            )
                            .await?;
                            let hb: Heartbeat = Heartbeat::new().await?;
                            let heartbeat_data = hb.create(heartbeat_data).await?;
                            println!("Heartbeat Data: {:?}", heartbeat_data);
                        }
                        RequestType::Logout => {
                            let logout_data = Logout::parse(
                                request_packet.payload_length,
                                &request_packet.payload,
                            )
                            .await?;
                            println!("Logout Data: {:?}", logout_data);
                        }
                        RequestType::Coordinates => {
                            let coordinates_data = Coordinates::parse(
                                request_packet.payload_length,
                                &request_packet.payload,
                            )
                            .await?;
                            let coordinates = Coordinates::new().await?;
                            let coordinates_data = coordinates.create(coordinates_data).await?;
                            println!("Coordinates Data: {:?}", coordinates_data);
                        }
                        _ => {
                            panic!("Invalid Request Type");
                        }
                    }
                }
                Err(error) => panic!("{:?}", error),
            }
        }
    }
}
