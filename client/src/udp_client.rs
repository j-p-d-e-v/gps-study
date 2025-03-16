use gps_tracker::actions::{Coordinates, Heartbeat, Login, Logout};
use gps_tracker::config::Config;
use gps_tracker::payload::Payload;
use gps_tracker::response::ResponseType;
use gps_tracker::RequestType;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use tokio::net::UdpSocket;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinatesItem {
    pub lon: String,
    pub lat: String,
}

#[derive(Debug, Clone)]
pub struct UdpClient {
    pub client_id: String,
    pub address: String,
    pub username: String,
    pub password: String,
    pub server_config: Config,
}

impl UdpClient {
    pub async fn new(
        address: String,
        username: String,
        password: String,
        server_config_path: String,
    ) -> Result<Self, String> {
        let server_config = Config::load(Some(server_config_path)).await?;

        Ok(Self {
            address,
            username,
            password,
            server_config,
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

    pub async fn load_coordinates_data(
        &self,
        path: String,
    ) -> Result<Vec<CoordinatesItem>, String> {
        match File::options().read(true).open(path) {
            Ok(mut file) => {
                let mut buf = String::new();
                if let Err(error) = file.read_to_string(&mut buf) {
                    return Err(error.to_string());
                }
                match serde_json::from_str(&buf) {
                    Ok(values) => Ok(values),
                    Err(error) => Err(error.to_string()),
                }
            }
            Err(error) => Err(error.to_string()),
        }
    }

    pub fn client_id_to_u32(value: Option<String>) -> Result<u32, String> {
        match value {
            Some(client_id) => match client_id.parse::<u32>() {
                Ok(value) => Ok(value),
                Err(error) => Err(format!("unable to parse client id, reason: {:?}", error)),
            },
            None => Err("invalid client id".to_string()),
        }
    }

    pub async fn simulate(
        &mut self,
        current_request_type: RequestType,
        client_id: Option<String>,
        coordinates: Option<CoordinatesItem>,
    ) -> Result<String, String> {
        let socket = self.launch().await?;
        match socket.connect(self.server_config.server.host.clone()).await {
            Ok(_) => {
                // Request Payload Generation and Sending
                match current_request_type {
                    RequestType::Login => {
                        println!("Sending Login as {}", self.username);
                        let payload_generator =
                            Login::generate_payload(self.username.clone(), self.password.clone())
                                .await?;
                        let payload_data = Payload::to_binary(&payload_generator)?;
                        if let Err(error) = socket.send(&payload_data).await {
                            return Err(format!("LOGIN REQUEST ERROR: {}", error.to_string()));
                        }
                    }
                    RequestType::HeartBeat => {
                        let client_id: u32 = Self::client_id_to_u32(client_id)?;
                        println!("Sending Hearbeat as {}", client_id);
                        let payload_generator = Heartbeat::generate_payload(client_id).await?;
                        let payload_data = Payload::to_binary(&payload_generator)?;
                        if let Err(error) = socket.send(&payload_data).await {
                            return Err(format!("HEARTBEAT REQUEST ERROR: {}", error.to_string()));
                        }
                    }
                    RequestType::Logout => {
                        let client_id: u32 = Self::client_id_to_u32(client_id)?;
                        println!("Sending Logout as {}", client_id);
                        let payload_generator = Logout::generate_payload(client_id).await?;
                        let payload_data = Payload::to_binary(&payload_generator)?;
                        if let Err(error) = socket.send(&payload_data).await {
                            return Err(format!("LOGOUT REQUEST ERROR: {}", error.to_string()));
                        }
                    }
                    RequestType::Coordinates => {
                        let client_id: u32 = Self::client_id_to_u32(client_id)?;
                        let mut lon: f64 = 0.0;
                        let mut lat: f64 = 0.0;
                        if let Some(item) = coordinates {
                            lon = if let Ok(value) = item.lon.parse::<f64>() {
                                value
                            } else {
                                return Err("unable to parse lon coordinates".to_string());
                            };
                            lat = if let Ok(value) = item.lat.parse::<f64>() {
                                value
                            } else {
                                return Err("unable to parse lat coordinates".to_string());
                            };
                        }
                        println!("Sending Coordinates as {}, {},{}", client_id, lon, lat);
                        let payload_generator =
                            Coordinates::generate_payload(client_id, lat, lon).await?;
                        let payload_data = Payload::to_binary(&payload_generator)?;
                        if let Err(error) = socket.send(&payload_data).await {
                            return Err(format!(
                                "COORDINATES REQUEST ERROR: {}",
                                error.to_string()
                            ));
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
                        Some(_) => {
                            if let Some(status) = filled.get(1) {
                                let _ = self
                                    .check_response_status(status, "login failed".to_string())
                                    .await?;
                            }
                            let client_id_filled: &[u8] = if let Some(id) = filled.get(2..) {
                                id
                            } else {
                                return Err("invalid client id".to_string());
                            };
                            let client_id = String::from_utf8_lossy(client_id_filled).to_string();
                            println!("Response received for {}", client_id);
                            return Ok(client_id);
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
