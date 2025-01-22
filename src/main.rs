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
    fn length(self) -> u8 {
        match self {
            Self::Invalid => 0x000,
            Self::Login => 0x0014,
            Self::Coordinates => 0x0010,
            Self::HeartBeat => 0x0004,
            Self::Logout => 0x0004,
        }
    }

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
struct LoginData {
    username: String,
    password: String,
}

fn main() -> Result<(), String> {
    let socket = UdpSocket::bind("127.0.0.1:34256").unwrap();
    println!("{:?}", RequestType::Login as u8);
    println!("{:?}", RequestType::Login.length());
    loop {
        let mut buf = [0; 64];
        let (amt, src) = socket.recv_from(&mut buf).unwrap();

        let request_type: RequestType = RequestType::get_by_value(buf[0]);

        println!("{:?}", amt);
        println!("{:?}", src);
        println!("{:?}", buf);
        println!("Request Type: {:?}", request_type);
    }
    // Ok(())
}
