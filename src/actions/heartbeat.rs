use crate::db::Db;
use crate::payload::Payload;
use crate::request::RequestType;
use crate::response::ResponseType;
use crate::user::{User, UserData};
use crate::validation::ValidationError;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;
use surrealdb::RecordId;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HeartbeatData {
    pub timestamp: Datetime,
    pub id: Option<RecordId>,
    pub source_address: String,
    pub user: RecordId,
}

#[derive(Debug)]
pub struct Heartbeat {
    db: Db,
}
/// Format:
/// Type: 0x03
/// Payload Length: 0x0004 (4 bytes or greater for client id)
/// Payload: <client_id> Example: 24564
///
/// Example:
/// ```
/// 03 00 04 00 00 5F F4
/// ```

impl Heartbeat {
    /// Generate Payload
    pub async fn generate_payload(client_id: u32) -> Result<String, String> {
        let hex_client_id: String = format!("{:08x}", client_id);
        let request_type = format!("{:02x}", RequestType::HeartBeat.to_value());
        let payload_length = format!("{:04x}", RequestType::HeartBeat.get_length());

        Ok(Payload::apply_spacing(
            format!("{}{}{}", request_type, payload_length, hex_client_id).as_str(),
        ))
    }

    pub async fn generate_response(client_id: String, is_error: bool) -> Result<String, String> {
        let hex_client_id: String = hex::encode_upper(client_id);
        println!("Hex Client Id: {}", hex_client_id);
        let request_type = format!("{:02x}", RequestType::HeartBeat.to_value());
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

    /// Initializes Heartbeat instance including database connections.
    pub async fn new() -> Result<Self, String> {
        let db = Db::connect().await?;
        Ok(Self { db })
    }

    /// Parse a heartbeat from a packet
    pub async fn parse(
        source_address: String,
        payload_length: usize,
        data: &[u8],
    ) -> Result<HeartbeatData, String> {
        if data.len() < payload_length {
            return Err(ValidationError::InvalidHeartbeatPayload.to_string());
        }
        let client_id_hex: Vec<String> = data.iter().map(|x| format!("{:x}", x)).collect();
        println!("Client Id: {:?}", client_id_hex);
        match u32::from_str_radix(&client_id_hex.concat(), 16) {
            Ok(client_id) => {
                let user: User = User::new().await?;
                let user_data: UserData = user.get_by_client_id(client_id).await?;
                if let Some(user_id) = user_data.id {
                    Ok(HeartbeatData {
                        source_address,
                        id: None,
                        user: user_id,
                        timestamp: Datetime::from(Utc::now()),
                    })
                } else {
                    return Err(ValidationError::InvalidUserId.to_string());
                }
            }
            Err(_) => Err(ValidationError::InvalidClientId.to_string()),
        }
    }

    /// Returns the table name.
    fn get_table(&self) -> String {
        String::from("heartbeat")
    }

    /// Create a heartbeat record
    pub async fn create(&self, data: HeartbeatData) -> Result<HeartbeatData, String> {
        match self
            .db
            .client
            .insert::<Vec<HeartbeatData>>(self.get_table())
            .content(data.clone())
            .await
        {
            Ok(response) => {
                if let Some(record) = response.get(0) {
                    return Ok(record.to_owned());
                }
                return Err("heartbeat error: no record found".to_string());
            }
            Err(error) => Err(format!("heartbeat error: {:?}", error)),
        }
    }
}

#[cfg(test)]
mod test_heartbeat {
    use super::*;
    use std::str::FromStr;

    #[tokio::test]
    pub async fn test_generate_payload() {
        let client_id: u32 = 24564;
        let payload = Heartbeat::generate_payload(client_id).await;
        let test_value_hex = "03 00 04 00 00 5F F4";
        assert!(payload.is_ok(), "{:?}", payload.err());
        assert_eq!(test_value_hex, payload.clone().unwrap().as_str());

        let payload = Payload::to_binary(payload.unwrap().as_str());
        assert!(payload.is_ok(), "{:?}", payload.err());
    }

    #[tokio::test]
    pub async fn test_create() {
        let hb = Heartbeat::new().await;
        assert!(hb.is_ok(), "{:?}", hb.err());
        let hb = hb.unwrap();

        for _ in 0..10 {
            let hb_data = hb
                .create(HeartbeatData {
                    id: None,
                    source_address: Some("127.0.0.1".to_string()),
                    user: Some(RecordId::from_str("users:0dgt5u58j2jh3oq4xzbt").unwrap()),
                    timestamp: Datetime::from(Utc::now()),
                })
                .await;
            assert!(hb_data.is_ok(), "{:?}", hb_data.err());
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }
}
