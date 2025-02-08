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
}
