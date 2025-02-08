use crate::validation::ValidationError;
use chrono::{DateTime, Utc};
#[derive(Debug)]
pub struct Heartbeat {
    client_id: u32,
    timestamp: DateTime<Utc>,
}

impl Heartbeat {
    pub fn parse(payload_length: usize, data: &[u8]) -> Result<Self, ValidationError> {
        if data.len() > payload_length {
            return Err(ValidationError::InvalidHeartbeatPayload);
        }
        let client_id_hex: Vec<String> = data.iter().map(|x| format!("{:x}", x)).collect();
        match u32::from_str_radix(&client_id_hex.concat(), 16) {
            Ok(client_id) => Ok(Self {
                client_id,
                timestamp: Utc::now(),
            }),
            Err(_) => Err(ValidationError::InvalidClientId),
        }
    }
}
