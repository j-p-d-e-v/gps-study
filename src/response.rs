#[derive(Debug)]
pub enum ResponseType {
    Success = 0x06,
    Error = 0x07,
}

impl ResponseType {
    pub fn to_value(self) -> u8 {
        match self {
            Self::Success => 0x06,
            Self::Error => 0x07,
        }
    }
}
