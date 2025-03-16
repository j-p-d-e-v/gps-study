use crate::db::Db;
use crate::payload::Payload;
use crate::request::RequestType;
use crate::response::ResponseType;
use crate::validation::ValidationError;

#[derive(Debug)]
pub struct Logout {
    db: Db,
}
/// Format:
/// Type: 0x04
/// Payload Length: 0x0004 (4 bytes or greater for client id)
/// Payload: <client_id> Example: 24564
///
/// Example:
/// ```
/// 04 00 04 00 00 5F F4
/// ```

impl Logout {
    /// Generate Payload
    pub async fn generate_payload(client_id: u32) -> Result<String, String> {
        let hex_client_id: String = format!("{:08x}", client_id);
        let request_type = format!("{:02x}", RequestType::Logout.to_value());
        let payload_length = format!("{:04x}", RequestType::Logout.get_length());

        Ok(Payload::apply_spacing(
            format!("{}{}{}", request_type, payload_length, hex_client_id).as_str(),
        ))
    }

    pub async fn generate_response(client_id: String, is_error: bool) -> Result<String, String> {
        let hex_client_id: String = hex::encode_upper(client_id);
        let request_type = format!("{:02x}", RequestType::Logout.to_value());
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

    pub async fn parse(payload_length: usize, data: &[u8]) -> Result<u32, String> {
        if data.len() > payload_length {
            return Err(ValidationError::InvalidLogoutPayload.to_string());
        }

        let client_id_hex: Vec<String> = data.iter().map(|x| format!("{:x}", x)).collect();
        match u32::from_str_radix(&client_id_hex.concat(), 16) {
            Ok(client_id) => Ok(client_id),
            Err(_) => Err(ValidationError::InvalidClientId.to_string()),
        }
    }

    pub async fn logout(&self, client_id: u32) -> Result<bool, String> {
        if let Err(error) = self
            .db
            .client
            .query(
                r#"
                    DELETE FROM type::table($coordinates_table) WHERE `user`.`client_id`=$client_id;
                    DELETE FROM type::table($heartbeat_table) WHERE `user`.`client_id`=$client_id;
                "#,
            )
            .bind(("coordinates_table", "coordinates"))
            .bind(("heartbeat_table", "heartbeat"))
            .bind(("client_id", client_id))
            .await
        {
            return Err(error.to_string());
        }
        Ok(true)
    }
}

#[cfg(test)]
mod test_logout {
    use super::*;

    #[tokio::test]
    pub async fn test_generate_payload() {
        let client_id: u32 = 24564;
        let payload = Logout::generate_payload(client_id).await;
        let test_value_hex = "04 00 04 00 00 5F F4";
        assert!(payload.is_ok(), "{:?}", payload.err());
        assert_eq!(test_value_hex, payload.clone().unwrap().as_str());

        let payload = Payload::to_binary(payload.unwrap().as_str());
        assert!(payload.is_ok(), "{:?}", payload.err());
    }

    #[tokio::test]
    pub async fn test_logout() {
        let client_id: u32 = 24564;
        let logout: Result<Logout, String> = Logout::new().await;
        assert!(logout.is_ok(), "{:?}", logout.err());
        let result: Result<bool, String> = logout.unwrap().logout(client_id).await;
        assert!(result.is_ok(), "{:?}", result.err());
    }
}
