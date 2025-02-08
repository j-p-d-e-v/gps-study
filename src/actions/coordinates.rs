use crate::validation::ValidationError;
use chrono::{DateTime, Utc};
use ieee_754::IEEE754;
#[derive(Debug)]
pub struct Coordinates {
    client_id: u32,
    latitude: f64,
    longitude: f64,
    timestamp: DateTime<Utc>,
}

impl Coordinates {
    pub fn parse(payload_length: usize, data: &[u8]) -> Result<Self, ValidationError> {
        if data.len() < payload_length {
            return Err(ValidationError::InvalidCoordinatesPayload);
        }
        match data.get(0..4) {
            Some(client_id_binary) => {
                let client_id_hex: Vec<String> = client_id_binary
                    .iter()
                    .map(|x| format!("{:x}", x))
                    .collect();
                match u32::from_str_radix(&client_id_hex.concat(), 16) {
                    Ok(client_id) => {
                        let mut latitude_value: f64 = 0.0;
                        let mut longitude_value: f64 = 0.0;

                        match data.get(4..12) {
                            Some(value) => {
                                let ieee_754: IEEE754 = IEEE754::new(
                                    value
                                        .iter()
                                        .map(|x| x.to_owned() as u32)
                                        .collect::<Vec<u32>>(),
                                );
                                match ieee_754.to_64bit() {
                                    Ok(latitude) => {
                                        latitude_value = latitude;
                                        println!("Latitude: {}", latitude_value);
                                    }
                                    Err(_) => {
                                        return Err(ValidationError::UnableToParseLatitude);
                                    }
                                }
                            }
                            None => {
                                return Err(ValidationError::InvalidLatitude);
                            }
                        }

                        match data.get(12..20) {
                            Some(value) => {
                                let ieee_754: IEEE754 = IEEE754::new(
                                    value
                                        .iter()
                                        .map(|x| x.to_owned() as u32)
                                        .collect::<Vec<u32>>(),
                                );
                                match ieee_754.to_64bit() {
                                    Ok(longitude) => {
                                        longitude_value = longitude;
                                        println!("Longitude: {}", longitude_value);
                                    }
                                    Err(_) => {
                                        return Err(ValidationError::UnableToParseLongitude);
                                    }
                                }
                            }
                            None => {
                                return Err(ValidationError::InvalidLongitude);
                            }
                        }

                        Ok(Self {
                            client_id,
                            latitude: latitude_value,
                            longitude: longitude_value,
                            timestamp: Utc::now(),
                        })
                    }
                    Err(_) => Err(ValidationError::InvalidClientId),
                }
            }
            None => Err(ValidationError::ClientIdEmpty),
        }
    }
}
