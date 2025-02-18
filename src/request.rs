use crate::validation::ValidationError;

#[derive(Debug, PartialEq)]
pub enum RequestType {
    Login = 0x01,
    Coordinates = 0x02,
    HeartBeat = 0x03,
    Logout = 0x04,
    Invalid = 0x00,
}

impl RequestType {
    pub fn to_value(self) -> u8 {
        match self {
            Self::Login => 0x01,
            Self::Coordinates => 0x02,
            Self::HeartBeat => 0x03,
            Self::Logout => 0x04,
            Self::Invalid => 0x00,
        }
    }

    pub fn get_length(self) -> u8 {
        match self {
            Self::Login => 0x0014,
            Self::Coordinates => 0x0004,
            Self::HeartBeat => 0x0004,
            Self::Logout => 0x00,
            Self::Invalid => 0x00,
        }
    }

    pub fn get_by_value(value: u8) -> RequestType {
        match value {
            0x01 => RequestType::Login,
            0x02 => RequestType::Coordinates,
            0x03 => RequestType::HeartBeat,
            0x04 => RequestType::Logout,
            _ => RequestType::Invalid,
        }
    }
}

#[derive(Debug)]
pub struct RequestPacket {
    pub request_type: RequestType,
    pub payload_length: usize,
    pub payload: Vec<u8>,
}

impl RequestPacket {
    pub fn parse(data: &[u8]) -> Result<Self, ValidationError> {
        match data.first() {
            Some(value) => {
                let request_type: RequestType = RequestType::get_by_value(value.to_owned());
                match data.get(2) {
                    Some(value) => match format!("{}", value.to_owned()).parse() {
                        Ok(payload_length) => match data.get(3..) {
                            Some(value) => Ok(Self {
                                request_type,
                                payload_length,
                                payload: value.to_vec(),
                            }),
                            None => Err(ValidationError::InvalidRequestPacketPayload),
                        },
                        Err(_) => Err(ValidationError::UnableToParseRequestPayloadLength),
                    },
                    None => Err(ValidationError::InvalidRequestPacketPayloadLength),
                }
            }
            None => Err(ValidationError::InvalidRequestPacket),
        }
    }
}
