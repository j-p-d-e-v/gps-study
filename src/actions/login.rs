use crate::validation::ValidationError;

#[derive(Debug)]
pub struct Login {
    username: String,
    password: String,
}

impl Login {
    pub fn parse(payload_length: usize, credentials: &[u8]) -> Result<Self, ValidationError> {
        if credentials.len() < payload_length {
            return Err(ValidationError::InvalidLoginPayload);
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
}
