use crate::db::Db;
use crate::payload::Payload;
use crate::request::RequestType;
use crate::response::ResponseType;
use crate::user::{User, UserData};
use crate::validation::ValidationError;
use chrono::Utc;
use ieee_754::IEEE754;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;
use surrealdb::RecordId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinatesData {
    pub user: RecordId,
    pub latitude: f64,
    pub longitude: f64,
    pub timestamp: Datetime,
}
// Format:
// Type: 0x02
// Payload Length: 0x0010
// Payload Format:
// - client_id = 4 bytes
// - latitude = 8 bytes
// - longitude = 8 bytes
// Example:
// ```
// 03 00 04 00 00 5F F4 40 24 00 01 4F 8B 58 8E C0 5F C0 00 04 31 BD E8
// ```

#[derive(Debug)]
pub struct Coordinates {
    db: Db,
}

impl Coordinates {
    /// Initializes Heartbeat instance including database connections.
    pub async fn new() -> Result<Self, String> {
        let db = Db::connect().await?;
        Ok(Self { db })
    }

    /// Generate Payload
    pub async fn generate_payload(
        client_id: u32,
        latitude: f64,
        longitude: f64,
    ) -> Result<String, String> {
        let hex_client_id: String = format!("{:08x}", client_id);
        let request_type = format!("{:02x}", RequestType::Coordinates.to_value());
        let payload_length = format!("{:04x}", RequestType::Coordinates.get_length());
        let latitude_hex: String = if let Ok(v) = IEEE754::to_64bit_hex(latitude) {
            v
        } else {
            return Err("unable to parse latitude and convert to hex".to_string());
        };
        let longitude_hex: String = if let Ok(v) = IEEE754::to_64bit_hex(longitude) {
            v
        } else {
            return Err("unable to parse longtitude and convert to hex".to_string());
        };

        Ok(Payload::apply_spacing(
            format!(
                "{}{}{}{}{}",
                request_type, payload_length, hex_client_id, latitude_hex, longitude_hex
            )
            .as_str(),
        ))
    }

    pub async fn generate_response(client_id: String, is_error: bool) -> Result<String, String> {
        let hex_client_id: String = hex::encode_upper(client_id);
        let request_type = format!("{:02x}", RequestType::Coordinates.to_value());
        let response_status = if is_error {
            ResponseType::Error.to_value()
        } else {
            ResponseType::Success.to_value()
        };
        let with_spacing = Payload::apply_spacing(
            format!("{}{:02x}{}", request_type, response_status, hex_client_id).as_str(),
        );
        Ok(with_spacing)
    }

    pub async fn parse(payload_length: usize, data: &[u8]) -> Result<CoordinatesData, String> {
        if data.len() < payload_length {
            return Err(ValidationError::InvalidCoordinatesPayload.to_string());
        }
        match data.get(0..4) {
            Some(client_id_binary) => {
                let client_id_hex: Vec<String> = client_id_binary
                    .iter()
                    .map(|x| format!("{:x}", x))
                    .collect();
                match u32::from_str_radix(&client_id_hex.concat(), 16) {
                    Ok(client_id) => {
                        let user: User = User::new().await?;
                        println!("Client Id:{:?}", client_id);
                        let user_data: UserData = user.get_by_client_id(client_id).await?;
                        let user_id = if let Some(user_id) = user_data.id {
                            user_id
                        } else {
                            return Err("invalid user id".to_string());
                        };

                        let mut coordinates_data: CoordinatesData = CoordinatesData {
                            longitude: 0.0,
                            latitude: 0.0,
                            user: user_id,
                            timestamp: Datetime::from(Utc::now()),
                        };

                        match data.get(4..12) {
                            Some(value) => {
                                let latitude: Result<f64, _> = IEEE754::to_64bit_float(
                                    value
                                        .iter()
                                        .map(|x| x.to_owned() as u32)
                                        .collect::<Vec<u32>>(),
                                );
                                match latitude {
                                    Ok(v) => {
                                        coordinates_data.latitude = v;
                                        println!("Latitude: {}", coordinates_data.latitude);
                                    }
                                    Err(_) => {
                                        return Err(
                                            ValidationError::UnableToParseLatitude.to_string()
                                        );
                                    }
                                }
                            }
                            None => {
                                return Err(ValidationError::InvalidLatitude.to_string());
                            }
                        }

                        match data.get(12..20) {
                            Some(value) => {
                                let longitude: Result<f64, _> = IEEE754::to_64bit_float(
                                    value
                                        .iter()
                                        .map(|x| x.to_owned() as u32)
                                        .collect::<Vec<u32>>(),
                                );
                                match longitude {
                                    Ok(v) => {
                                        coordinates_data.longitude = v;
                                        println!("Longitude: {}", coordinates_data.longitude);
                                    }
                                    Err(_) => {
                                        return Err(
                                            ValidationError::UnableToParseLongitude.to_string()
                                        );
                                    }
                                }
                            }
                            None => {
                                return Err(ValidationError::InvalidLongitude.to_string());
                            }
                        }
                        Ok(coordinates_data.to_owned())
                    }
                    Err(_) => Err(ValidationError::InvalidClientId.to_string()),
                }
            }
            None => Err(ValidationError::ClientIdEmpty.to_string()),
        }
    }

    /// Returns the table name.
    fn get_table(&self) -> String {
        String::from("coordinates")
    }

    /// Create a coordinates record
    pub async fn create(&self, data: CoordinatesData) -> Result<CoordinatesData, String> {
        match self
            .db
            .client
            .insert::<Vec<CoordinatesData>>(self.get_table())
            .content(data.clone())
            .await
        {
            Ok(response) => {
                if let Some(record) = response.get(0) {
                    return Ok(record.to_owned());
                }
                return Err("coordinates error: no record found".to_string());
            }
            Err(error) => Err(format!("coordinates error: {:?}", error)),
        }
    }
}

#[cfg(test)]
mod test_coordinates {
    use super::*;
    use std::str::FromStr;

    #[tokio::test]
    pub async fn test_generate_payload() {
        let client_id: u32 = 24564;
        let latitude: f64 = 10.00001;
        let longitude: f64 = -127.000001;
        let payload = Coordinates::generate_payload(client_id, latitude, longitude).await;
        let test_value_hex = "02 00 04 00 00 5F F4 40 24 00 01 4F 8B 58 8E C0 5F C0 00 04 31 BD E8";
        assert!(payload.is_ok(), "{:?}", payload.err());
        assert_eq!(test_value_hex, payload.clone().unwrap().as_str());

        let payload = Payload::to_binary(payload.unwrap().as_str());
        assert!(payload.is_ok(), "{:?}", payload.err());
    }

    #[tokio::test]
    pub async fn test_create() {
        let coords = Coordinates::new().await;
        assert!(coords.is_ok(), "{:?}", coords.err());
        let coords = coords.unwrap();

        for _ in 0..10 {
            let coords_data = coords
                .create(CoordinatesData {
                    longitude: -127.000001,
                    latitude: 10.00001,
                    user: RecordId::from_str("users:0dgt5u58j2jh3oq4xzbt").unwrap(),
                    timestamp: Datetime::from(Utc::now()),
                })
                .await;
            assert!(coords_data.is_ok(), "{:?}", coords_data.err());
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }
}
