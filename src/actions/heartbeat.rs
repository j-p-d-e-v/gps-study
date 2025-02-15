use crate::db::Db;
use crate::user::{User, UserData};
use crate::validation::ValidationError;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HeartbeatData {
    timestamp: String,
    id: Option<RecordId>,
    source_address: Option<String>,
    user: Option<RecordId>,
}

#[derive(Debug)]
pub struct Heartbeat {
    db: Db,
}

impl Heartbeat {
    pub async fn new() -> Result<Self, String> {
        let db = Db::connect().await?;
        Ok(Self { db })
    }

    pub async fn parse(
        source_address: String,
        payload_length: usize,
        data: &[u8],
    ) -> Result<HeartbeatData, String> {
        if data.len() > payload_length {
            return Err(ValidationError::InvalidHeartbeatPayload.to_string());
        }
        let client_id_hex: Vec<String> = data.iter().map(|x| format!("{:x}", x)).collect();
        match u32::from_str_radix(&client_id_hex.concat(), 16) {
            Ok(client_id) => {
                let user: User = User::new().await?;
                let user_data: UserData = user.get_by_client_id(client_id).await?;
                Ok(HeartbeatData {
                    source_address: Some(source_address),
                    id: None,
                    user: user_data.id,
                    timestamp: Utc::now().to_rfc2822(),
                })
            }
            Err(_) => Err(ValidationError::InvalidClientId.to_string()),
        }
    }

    fn get_table(&self) -> String {
        String::from("heartbeat")
    }

    pub async fn create(&self, data: HeartbeatData) -> Result<(), String> {
        match self
            .db
            .client
            .insert::<Vec<HeartbeatData>>(self.get_table())
            .content(data.clone())
            .await
        {
            Ok(response) => {
                println!("{:?}", response);
                Ok(())
            }
            Err(error) => Err(format!("heartbeat error: {:?}", error)),
        }
    }
}

#[cfg(test)]
mod test_heartbeat {
    use std::str::FromStr;

    use super::*;
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
                    timestamp: Utc::now().to_rfc3339(),
                })
                .await;
            assert!(hb_data.is_ok(), "{:?}", hb_data.err());
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }
}
