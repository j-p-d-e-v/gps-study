use crate::actions::{Coordinates, Heartbeat, Login, Logout};
use crate::config::Config;
use crate::payload::Payload;
use crate::{RequestPacket, RequestType};
use std::net::UdpSocket;

#[derive(Debug)]
pub struct UdpServer;

impl UdpServer {
    pub async fn launch() -> Result<(), String> {
        let config: Config = Config::load(None).await?;
        let server_config = config.server;
        println!("Server: {}", server_config.host);
        let socket = UdpSocket::bind(server_config.host).unwrap();
        loop {
            let mut buf = [0; 64];
            let (size, source_address) = socket.recv_from(&mut buf).unwrap();
            let filled = &mut buf[..size];
            println!("Filled: {:?}", String::from_utf8(filled.to_vec()));
            match RequestPacket::parse(filled) {
                Ok(request_packet) => {
                    println!("Request Packet: {:x?}", request_packet);
                    match request_packet.request_type {
                        RequestType::Login => {
                            let login_data = Login::parse(
                                request_packet.payload_length,
                                &request_packet.payload,
                            )
                            .await?;

                            println!("Login Data: {:?}", login_data);
                            let mut response_data: String = String::new();
                            match Login::authenticate(login_data).await {
                                Ok(user_data) => {
                                    response_data = Login::generate_response(
                                        user_data.client_id.to_string(),
                                        false,
                                    )
                                    .await?;
                                    println!("User Data: {:?}", user_data);
                                    println!("Reponse Data: {:?}", response_data);
                                    println!("Source Address: {:?}", source_address);
                                }
                                Err(error) => {
                                    response_data =
                                        Login::generate_response("0".to_string(), true).await?;
                                    eprintln!("{}", error);
                                }
                            }

                            match socket.connect(source_address) {
                                Ok(_) => {
                                    let response_binary =
                                        Payload::to_binary(response_data.as_str())?;
                                    println!("Binary Data: {:?}", response_binary);
                                    if let Err(error) = socket.send(&response_binary) {
                                        eprintln!(
                                            "unable to send login response, reason: {}",
                                            error.to_string()
                                        );
                                    }
                                }
                                Err(error) => {
                                    eprintln!("login send connect error {}", error.to_string());
                                }
                            }
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
                            eprint!("Invalid Request Type");
                        }
                    }
                }
                Err(error) => eprint!("{:?}", error),
            }
        }
    }
}
