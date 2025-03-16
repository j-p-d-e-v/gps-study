use crate::actions::{Coordinates, Heartbeat, Login, Logout};
use crate::config::Config;
use crate::payload::Payload;
use crate::user::User;
use crate::{RequestPacket, RequestType};
use std::net::{SocketAddr, UdpSocket};

#[derive(Debug)]
pub struct UdpServer;

impl UdpServer {
    pub async fn respond(
        socket: &UdpSocket,
        source_address: SocketAddr,
        response_data: &str,
    ) -> Result<(), String> {
        let response_binary = Payload::to_binary(response_data)?;
        println!("Binary Data: {:?}", response_binary);
        if let Err(error) = socket.send_to(&response_binary, source_address) {
            return Err(format!(
                "unable to send login response, reason: {}",
                error.to_string()
            ));
        }
        Ok(())
    }
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
                            let response_data: String = match Login::authenticate(login_data).await
                            {
                                Ok(user_data) => {
                                    Login::generate_response(user_data.client_id.to_string(), false)
                                        .await?
                                }
                                Err(error) => {
                                    eprintln!("{}", error);
                                    Login::generate_response("0".to_string(), true).await?
                                }
                            };
                            if let Err(error) =
                                Self::respond(&socket, source_address, response_data.as_str()).await
                            {
                                eprint!("LOGIN RESPONSE ERROR: {}", error);
                            }
                        }
                        RequestType::HeartBeat => {
                            let heartbeat_data = Heartbeat::parse(
                                source_address.to_string(),
                                request_packet.payload_length,
                                &request_packet.payload,
                            )
                            .await?;

                            println!("Heartbeat Data: {:?}", heartbeat_data);
                            let hb: Heartbeat = Heartbeat::new().await?;
                            let response_data: String = match hb.create(heartbeat_data).await {
                                Ok(hb_data) => {
                                    let user = User::new().await?;
                                    match user.get_by_id(hb_data.user).await {
                                        Ok(user_data) => {
                                            Heartbeat::generate_response(
                                                user_data.client_id.to_string(),
                                                false,
                                            )
                                            .await?
                                        }
                                        Err(error) => {
                                            eprintln!("HEARTBEAT ERROR: {}", error);
                                            Heartbeat::generate_response(
                                                "000000000".to_string(),
                                                true,
                                            )
                                            .await?
                                        }
                                    }
                                }
                                Err(error) => {
                                    return Err(format!("{:?}", error));
                                }
                            };
                            if let Err(error) =
                                Self::respond(&socket, source_address, response_data.as_str()).await
                            {
                                eprint!("HEART BEAT RESPONSE ERROR: {}", error);
                            }
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
