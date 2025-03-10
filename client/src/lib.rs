use gps_tracker::actions::Login;
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

    pub async fn simulate(&mut self) -> Result<(), String> {
        let payload_generator =
            Login::generate_payload(self.username.clone(), self.password.clone()).await?;
        let payload_data = Payload::to_binary(&payload_generator)?;
        let socket = self.launch().await?;
        match socket.connect(self.config.server.host.clone()).await {
            Ok(_) => {
                if let Err(error) = socket.send(&payload_data).await {
                    assert!(false, "{:?}", error);
                }
                loop {
                    let mut buf = [0; 64];
                    let (size, _) = socket.recv_from(&mut buf).await.unwrap();
                    if size > 0 {
                        let filled = &mut buf[..size];
                        match filled.get(0) {
                            Some(request_type) => {
                                if request_type == &RequestType::Login.to_value() {
                                    println!("Login Response");
                                    if let Some(status) = filled.get(1) {
                                        let _ = self
                                            .check_response_status(
                                                status,
                                                "login failed".to_string(),
                                            )
                                            .await?;
                                    }
                                    self.client_id = String::from_utf8(filled.to_vec()).unwrap();
                                    println!("Client Id: {}", self.client_id);
                                }
                            }
                            None => {}
                        }
                    } else {
                        panic!("nothing is received");
                    }
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
            "0.0.0.0:8086".to_string(),
            "test1".to_string(),
            "notsecurepassword".to_string(),
        )
        .await;
        assert!(client.is_ok(), "{:?}", client.err());
        let mut client: UdpClient = client.unwrap();
        let simulation: Result<(), String> = client.simulate().await;
        assert!(simulation.is_ok(), "{:?}", simulation.err());
    }
}
