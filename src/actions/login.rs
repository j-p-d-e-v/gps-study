use crate::payload::Payload;
use crate::request::RequestType;
use crate::user::{User, UserData};
use crate::validation::ValidationError;

#[derive(Debug)]
pub struct Login {
    username: String,
    password: String,
}
/// Format:
/// Type: 0x01
/// Payload Length: 0x0014 (20 bytes, 10 bytes for username and 10 bytes for password)
/// Payload: <username> <password> example: root notsecurepassword
///
/// Example:
/// ```
/// 01 00 14 72 6F 6F 74 00 6E 6F 74 73 65 63 75 72 65 70 61 73 73 77 6F 72 64
/// ```
impl Login {
    /// Generate Payload
    pub async fn generate_payload(username: String, password: String) -> Result<String, String> {
        let hex_username: String = hex::encode_upper(username);
        let hex_password: String = hex::encode(password);
        let request_type = format!("{:02x}", RequestType::Login.to_value());
        let payload_length = format!("{:04x}", RequestType::Login.get_length());

        Ok(Payload::apply_spacing(
            format!(
                "{}{}{}00{}",
                request_type, payload_length, hex_username, hex_password
            )
            .as_str(),
        ))
    }

    pub async fn generate_response() -> Result<String, String> {
        todo!("not implemented");
    }

    /// Parse the login packet
    pub async fn parse(payload_length: usize, credentials: &[u8]) -> Result<Self, String> {
        if credentials.len() < payload_length {
            return Err(ValidationError::InvalidLoginPayload.to_string());
        }
        let mut username: Vec<u8> = Vec::new();
        let mut password: Vec<u8> = Vec::new();
        let mut separator_flag: bool = false;

        for value in credentials {
            if value == &0 {
                separator_flag = true;
                continue;
            }
            if separator_flag {
                password.push(value.to_owned());
            } else {
                username.push(value.to_owned());
            }
        }

        Ok(Self {
            username: String::from_utf8(username).unwrap(),
            password: String::from_utf8(password).unwrap(),
        })
    }

    /// authenticate a user
    pub async fn authenticate(credential: Login) -> Result<UserData, String> {
        let user: User = User::new().await?;
        let data: UserData = user
            .get_by_username_and_password(&credential.username, &credential.password)
            .await?;
        Ok(data)
    }
}

#[cfg(test)]
mod test_login {
    use super::*;

    #[tokio::test]
    async fn test_payload_generator() {
        let username = "root".to_string();
        let password = "notsecurepassword".to_string();
        let test_value =
            "01 00 14 72 6F 6F 74 00 6E 6F 74 73 65 63 75 72 65 70 61 73 73 77 6F 72 64";
        let payload = Login::generate_payload(username, password).await;
        assert!(payload.is_ok(), "{:?}", payload.err());
        assert_eq!(test_value.to_string(), payload.unwrap());
        let test_value_binary: &[u8] = &[
            1, 0, 20, 114, 111, 111, 116, 0, 110, 111, 116, 115, 101, 99, 117, 114, 101, 112, 97,
            115, 115, 119, 111, 114, 100,
        ];
        let payload: Result<Vec<u8>, String> = Payload::to_binary(test_value);
        assert!(payload.is_ok(), "{:?}", payload.err());
        assert_eq!(test_value_binary, payload.unwrap());
    }

    #[tokio::test]
    async fn test_authentication() {
        let username = "root".to_string();
        let password = "notsecurepassword".to_string();

        let user_data: Result<UserData, String> =
            Login::authenticate(Login { username, password }).await;
        assert!(user_data.is_ok(), "{:?}", user_data.err());
    }
}
