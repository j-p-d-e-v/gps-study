#[derive(Debug)]
pub enum ValidationError {
    InvalidLogin,
    InvalidLoginPayload,
    InvalidClientId,
    InvalidLogoutPayload,
    InvalidHeartbeatPayload,
    InvalidCoordinatesPayload,
    InvalidLatitude,
    InvalidLongitude,
    ClientIdEmpty,
    UnableToParseLatitude,
    UnableToParseLongitude,
    InvalidRequestPacket,
    InvalidRequestPacketPayloadLength,
    InvalidRequestPacketPayload,
    UnableToParseRequestPayloadLength,
    InvalidUserId,
}

impl ValidationError {
    pub fn to_hex() -> String {
        todo!("not implemented");
    }
}
impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ValidationError::InvalidLogin => "Invalid login attempt",
            ValidationError::InvalidLoginPayload => "Invalid login payload",
            ValidationError::InvalidClientId => "Invalid client ID",
            ValidationError::InvalidLogoutPayload => "Invalid logout payload",
            ValidationError::InvalidHeartbeatPayload => "Invalid heartbeat payload",
            ValidationError::InvalidCoordinatesPayload => "Invalid coordinates payload",
            ValidationError::InvalidLatitude => "Invalid latitude value",
            ValidationError::InvalidLongitude => "Invalid longitude value",
            ValidationError::ClientIdEmpty => "Client ID is empty",
            ValidationError::UnableToParseLatitude => "Unable to parse latitude",
            ValidationError::UnableToParseLongitude => "Unable to parse longitude",
            ValidationError::InvalidRequestPacket => "Invalid request packet",
            ValidationError::InvalidRequestPacketPayloadLength => {
                "Invalid request packet payload length"
            }
            ValidationError::InvalidRequestPacketPayload => "Invalid request packet payload",
            ValidationError::UnableToParseRequestPayloadLength => {
                "Unable to parse request payload length"
            }
            Self::InvalidUserId => "Invalid UserId",
        };
        write!(f, "{}", message)
    }
}
