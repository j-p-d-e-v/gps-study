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
    /*
    * fn length(self) -> u8 {
        match self {
            Self::Invalid => 0x000,
            Self::Login => 0x0014,
            Self::Coordinates => 0x0010,
            Self::HeartBeat => 0x0004,
            Self::Logout => 0x0004,
        }
    }*/

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

impl LoginData {
    fn get(data: &[u8]) -> Self {
        let payload_length: usize =
            format!("{}", data.get(1).expect("Payload Length is expected."))
                .parse()
                .unwrap();
        println!("Login Data Length: {:?}", payload_length);
        let credentials: &[u8] = data.get(2..).unwrap();

        if credentials.len() < payload_length {
            panic!(
                "Username and Password length must not be less than {}",
                payload_length
            );
        }
        let mut username: String = String::new();
        let mut password: String = String::new();

        for value in credentials {
            let i: char = format!("{:x}", value).parse().unwrap();
            println!("{}", i);
        }
        println!("{:x?}", data.get(2..));
        Self {
            username: String::new(),
            password: String::new(),
        }
    }
}

fn main() -> Result<(), String> {
    let socket = UdpSocket::bind("127.0.0.1:34256").unwrap();
    loop {
        let mut buf = [0; 64];
        let (size, src) = socket.recv_from(&mut buf).unwrap();
        let filled = &mut buf[..size];
        let request_type: RequestType = RequestType::get_by_value(filled[0]);

        match request_type {
            RequestType::Login => {
                let login_data = LoginData::get(&filled[1..]);
                println!("Login Data: {:?}", login_data);
                println!("Login Request Type");
            }
            _ => {
                panic!("Invalid Request Type");
            }
        }

        println!("Size: {:?}", size);
        println!("Src: {:?}", src);
        println!("Filled {:X?}", filled);
        println!("Request Type: {:?}", request_type);
    }
    // Ok(())
}
