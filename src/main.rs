use chrono::{DateTime, Utc};
use ieee_754::IEEE754;
use std::net::UdpSocket;

#[derive(Debug, PartialEq)]
enum RequestType {
    Login = 0x01,
    Coordinates = 0x02,
    HeartBeat = 0x03,
    Logout = 0x04,
    Invalid = 0x00,
}

impl RequestType {
    fn get_by_value(value: u8) -> RequestType {
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
struct RequestPacket {
    request_type: RequestType,
    payload_length: usize,
    payload: Vec<u8>,
}

impl RequestPacket {
    fn parse(data: &[u8]) -> Self {
        let request_type: RequestType = RequestType::get_by_value(data.get(0).unwrap().to_owned());
        let payload_length: usize = format!("{}", data.get(2).unwrap().to_owned())
            .parse()
            .unwrap();
        let payload: Vec<u8> = data.get(3..).unwrap().to_vec();

        Self {
            request_type,
            payload_length,
            payload,
        }
    }
}

#[derive(Debug)]
struct LoginData {
    username: String,
    password: String,
}

impl LoginData {
    fn parse(payload_length: usize, credentials: &[u8]) -> Self {
        if credentials.len() < payload_length {
            panic!(
                "Username and Password length must not be less than {}",
                payload_length
            );
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

        Self {
            username: String::from_utf8(username).unwrap(),
            password: String::from_utf8(password).unwrap(),
        }
    }
}

#[derive(Debug)]
struct HeartBeatData {
    client_id: u32,
    timestamp: DateTime<Utc>,
}

impl HeartBeatData {
    fn parse(payload_length: usize, data: &[u8]) -> Self {
        if data.len() > payload_length {
            panic!("Payload Length for heartbeat exceeded.")
        }
        let client_id_hex: Vec<String> = data.iter().map(|x| format!("{:x}", x)).collect();
        let client_id: u32 = u32::from_str_radix(&client_id_hex.concat(), 16).unwrap();

        Self {
            client_id,
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug)]
struct LogoutData {
    client_id: u32,
    timestamp: DateTime<Utc>,
}

impl LogoutData {
    fn parse(payload_length: usize, data: &[u8]) -> Self {
        if data.len() > payload_length {
            panic!("Payload Length for logout exceeded.")
        }

        let client_id_hex: Vec<String> = data.iter().map(|x| format!("{:x}", x)).collect();
        let client_id: u32 = u32::from_str_radix(&client_id_hex.concat(), 16).unwrap();

        Self {
            client_id,
            timestamp: Utc::now(),
        }
    }
}

#[derive(Debug)]
struct CoordinatesData {
    client_id: u32,
    latitude: f64,
    longitude: f64,
    timestamp: DateTime<Utc>,
}

impl CoordinatesData {
    fn parse(payload_length: usize, data: &[u8]) -> Self {
        if data.len() < payload_length {
            panic!("Data length must be greater or equal to payload length");
        }
        let client_id_str: &[u8] = data.get(0..4).unwrap();
        let client_id_hex: Vec<String> = client_id_str.iter().map(|x| format!("{:x}", x)).collect();
        let client_id: u32 = u32::from_str_radix(&client_id_hex.concat(), 16).unwrap();

        let latitude_str: &[u8] = data.get(4..12).unwrap();
        let longiture_str: &[u8] = data.get(12..20).unwrap();

        println!("longitude String {:?}", longiture_str);

        let lat_iee754: IEEE754 = IEEE754::new(
            latitude_str
                .iter()
                .map(|x| x.to_owned() as u32)
                .collect::<Vec<u32>>(),
        );

        let lat_value = lat_iee754.to_64bit().unwrap();
        println!("Latitude Value: {}", lat_value);

        let long_iee754: IEEE754 = IEEE754::new(
            longiture_str
                .iter()
                .map(|x| x.to_owned() as u32)
                .collect::<Vec<u32>>(),
        );
        let long_value = long_iee754.to_64bit().unwrap();
        println!("longitude Value: {}", long_value);
        Self {
            client_id,
            latitude: lat_value,
            longitude: long_value,
            timestamp: Utc::now(),
        }
    }
}

/* Notes: To be remove later
 * printf "02 00 10 12 34 56 78 C0 52 AF BE 04 89 76 8E 40 66 33 4F 5C 29 C0 5C" | xxd -r -p > coordinates_packet.bin
 * 02 00 10 12 34 56 78 C0 52 AF BE 04 89 76 8E 40 66 33 4F 5C 29 C0 5C
 * */
fn main() -> Result<(), String> {
    let server_addr = "127.0.0.1:34256";
    let socket = UdpSocket::bind(server_addr).unwrap();
    println!("Server: {}", server_addr);
    loop {
        let mut buf = [0; 64];
        let (size, src) = socket.recv_from(&mut buf).unwrap();
        let filled = &mut buf[..size];
        let request_packet: RequestPacket = RequestPacket::parse(filled);
        println!("Request Packet: {:?}", request_packet);

        match request_packet.request_type {
            RequestType::Login => {
                let login_data =
                    LoginData::parse(request_packet.payload_length, &request_packet.payload);
                println!("Login Data: {:?}", login_data);
                println!("Login Request Type");
            }
            RequestType::HeartBeat => {
                let heartbeat_data =
                    HeartBeatData::parse(request_packet.payload_length, &request_packet.payload);
                println!("Heartbeat Data: {:?}", heartbeat_data);
                println!("HeartBeat Request Type");
            }
            RequestType::Logout => {
                let logout_data =
                    LogoutData::parse(request_packet.payload_length, &request_packet.payload);
                println!("Logout Data: {:?}", logout_data);
                println!("Logout Request Type");
            }
            RequestType::Coordinates => {
                let coordinates_data =
                    CoordinatesData::parse(request_packet.payload_length, &request_packet.payload);
                println!("Coordinates Data: {:?}", coordinates_data);
                println!("Coordinates Request Type");
            }
            _ => {
                panic!("Invalid Request Type");
            }
        }

        println!("Size: {:?}", size);
        println!("Src: {:?}", src);
    }
    // Ok(())
}
