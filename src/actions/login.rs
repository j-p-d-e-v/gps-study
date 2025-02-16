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
/// 01 00 14 72 6F 6F 74 00 00 00 00 00 00 6E 6F 74 73 65 63 75 72 65 70 61 73 73 77 6F 72 64
/// ```
impl Login {
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
    async fn test_authentication() {
        let username = "root".to_string();
        let password = "notsecurepassword".to_string();

        let user_data: Result<UserData, String> =
            Login::authenticate(Login { username, password }).await;
        assert!(user_data.is_ok(), "{:?}", user_data.err());
    }
}
