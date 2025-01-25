use chrono::{DateTime, Utc};
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
    payload_length: u8,
    payload: Vec<u8>,
}

impl RequestPacket {
    fn parse(data: &[u8]) -> Self {
        let request_type: RequestType = RequestType::get_by_value(data.get(0).unwrap().to_owned());
        let payload_length: u8 = data.get(2).unwrap().to_owned();
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
    fn parse(payload_length: u8, credentials: &[u8]) -> Self {
        let length: usize = format!("{}", payload_length).parse().unwrap();

        if credentials.len() < length {
            panic!(
                "Username and Password length must not be less than {}",
                length
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
    fn parse() -> Self {
        todo!("HeartBeat")
    }
}

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
                //let hb_data = LoginData::get(request_packet.payload_length, &request_packet.payload);
                //println!("Login Data: {:?}", login_data);
                println!("HeartBeat Request Type");
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
