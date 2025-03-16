use gps_tracker::actions::{Heartbeat, Login};
use gps_tracker::config::Config;
use gps_tracker::payload::Payload;
use gps_tracker::response::ResponseType;
use gps_tracker::RequestType;
use tokio::net::UdpSocket;

#[derive(Debug, Clone)]
pub struct UdpClient {
    pub client_id: String,
    pub address: String,
    pub username: String,
    pub password: String,
    pub config: Config,
}

impl UdpClient {
    pub async fn new(address: String, username: String, password: String) -> Result<Self, String> {
        let config = Config::load(Some("../config.toml".to_string())).await?;

        Ok(Self {
            address,
            username,
            password,
            config,
            client_id: String::new(),
        })
    }

    pub async fn launch(&self) -> Result<UdpSocket, String> {
        match UdpSocket::bind(self.address.clone()).await {
            Ok(socket) => Ok(socket),
            Err(error) => Err(format!(
                "unable to establish a udp server, reason: {:?}",
                error
            )),
        }
    }

    pub async fn check_response_status(
        &self,
        status: &u8,
        error_message: String,
    ) -> Result<(), String> {
        if status == &ResponseType::Error.to_value() {
            Err(format!("ERROR: {}", error_message))
        } else {
            Ok(())
        }
    }

    pub async fn simulate(
        &mut self,
        current_request_type: RequestType,
        client_id: Option<String>,
    ) -> Result<String, String> {
        let socket = self.launch().await?;
        match socket.connect(self.config.server.host.clone()).await {
            Ok(_) => {
                // Request Payload Generation and Sending
                match current_request_type {
                    RequestType::Login => {
                        println!("Login");
                        let payload_generator =
                            Login::generate_payload(self.username.clone(), self.password.clone())
                                .await?;
                        let payload_data = Payload::to_binary(&payload_generator)?;
                        if let Err(error) = socket.send(&payload_data).await {
                            return Err(format!("LOGIN REQUEST ERROR: {}", error.to_string()));
                        }
                    }
                    RequestType::HeartBeat => {
                        let client_id: u32 = if let Some(client_id) = client_id {
                            match client_id.parse::<u32>() {
                                Ok(value) => value,
                                Err(_) => {
                                    return Err(
                                        "unable to parse client id in heartbeat".to_string()
                                    );
                                }
                            }
                        } else {
                            return Err("invalid client id".to_string());
                        };
                        println!("Hearbeat: {}", client_id);
                        let payload_generator = Heartbeat::generate_payload(client_id).await?;
                        let payload_data = Payload::to_binary(&payload_generator)?;
                        if let Err(error) = socket.send(&payload_data).await {
                            return Err(format!("HEARTBEAT REQUEST ERROR: {}", error.to_string()));
                        }
                    }
                    _ => {
                        return Err("nothing to do".to_string());
                    }
                }

                // Response Data Receiving
                let mut buf = [0; 64];
                let (size, _) = socket.recv_from(&mut buf).await.unwrap();
                if size > 0 {
                    let filled = &mut buf[..size];
                    match filled.get(0) {
                        Some(request_type) => {
                            if let Some(status) = filled.get(1) {
                                let _ = self
                                    .check_response_status(status, "login failed".to_string())
                                    .await?;
                            }

                            if request_type == &RequestType::Login.to_value() {
                                println!("Login Response");
                                let client_id_filled = if let Some(id) = filled.get(2..) {
                                    id
                                } else {
                                    return Err("invalid client id".to_string());
                                };
                                let client_id =
                                    String::from_utf8_lossy(client_id_filled).to_string();
                                println!("Client Id: {}", client_id);
                                return Ok(client_id);
                            } else if request_type == &RequestType::HeartBeat.to_value() {
                                println!("Heartbeat Response");
                                let client_id_filled = if let Some(id) = filled.get(2..) {
                                    id
                                } else {
                                    return Err("invalid client id".to_string());
                                };
                                let client_id =
                                    String::from_utf8_lossy(client_id_filled).to_string();
                                println!("Client Id: {}", client_id);
                                return Ok(client_id);
                            } else {
                                return Err("unknown request type".to_string());
                            }
                        }
                        None => {
                            return Err("nothing to do".to_string());
                        }
                    }
                } else {
                    return Err("nothing is received".to_string());
                }
            }
            Err(error) => Err(error.to_string()),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client() {
        let client: Result<UdpClient, String> = UdpClient::new(
            "0.0.0.0:7082".to_string(),
            "test1".to_string(),
            "notsecurepassword".to_string(),
        )
        .await;
        assert!(client.is_ok(), "{:?}", client.err());
        let mut client: UdpClient = client.unwrap();
        let simulation: Result<String, String> = client.simulate(RequestType::Login, None).await;
        assert!(simulation.is_ok(), "{:?}", simulation.err());

        let client_id: String = simulation.unwrap().clone();
        let handler = tokio::spawn(async move {
            let client: Result<UdpClient, String> = UdpClient::new(
                "0.0.0.0:7086".to_string(),
                "test1".to_string(),
                "notsecurepassword".to_string(),
            )
            .await;
            assert!(client.is_ok(), "{:?}", client.err());
            let mut client: UdpClient = client.unwrap();

            loop {
                let simulation: Result<String, String> = client
                    .simulate(RequestType::HeartBeat, Some(client_id.clone()))
                    .await;
                assert!(simulation.is_ok(), "{:?}", simulation.err());
                std::thread::sleep(std::time::Duration::from_secs(3));
            }
        });
        handler.await.unwrap();
    }
}
