use crate::db::Db;
use crate::user::{User, UserData};
use crate::validation::ValidationError;
use chrono::Utc;
use ieee_754::IEEE754;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;
use surrealdb::RecordId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinatesData {
    user: Option<RecordId>,
    latitude: f64,
    longitude: f64,
    timestamp: Datetime,
}
/// Format:
/// Type: 0x03
/// Payload Length: 0x0004 (4 bytes or greater for client id)
/// Payload: <client_id> Example: 24564
///
/// Example:
/// ```
/// 02 00 04 00 00 6B 43 40 2D 4C F3 81 7F 57 D2 40 5E 42 FE CE 09 D7 FB
/// ```

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

                        let mut coordinates_data: CoordinatesData = CoordinatesData {
                            longitude: 0.0,
                            latitude: 0.0,
                            user: user_data.id,
                            timestamp: Datetime::from(Utc::now()),
                        };

                        match data.get(4..12) {
                            Some(value) => {
                                let ieee_754: IEEE754 = IEEE754::new(
                                    value
                                        .iter()
                                        .map(|x| x.to_owned() as u32)
                                        .collect::<Vec<u32>>(),
                                );
                                match ieee_754.to_64bit() {
                                    Ok(latitude) => {
                                        coordinates_data.latitude = latitude;
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
                                let ieee_754: IEEE754 = IEEE754::new(
                                    value
                                        .iter()
                                        .map(|x| x.to_owned() as u32)
                                        .collect::<Vec<u32>>(),
                                );
                                match ieee_754.to_64bit() {
                                    Ok(longitude) => {
                                        coordinates_data.longitude = longitude;
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
mod test_heartbeat {
    use super::*;
    use std::str::FromStr;

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
                    user: Some(RecordId::from_str("users:0dgt5u58j2jh3oq4xzbt").unwrap()),
                    timestamp: Datetime::from(Utc::now()),
                })
                .await;
            assert!(coords_data.is_ok(), "{:?}", coords_data.err());
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }
}
