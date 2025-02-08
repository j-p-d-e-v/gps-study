use gps_tracker::actions::{Coordinates, Heartbeat, Login, Logout};
use gps_tracker::{RequestPacket, RequestType};
use std::net::UdpSocket;
fn main() -> Result<(), String> {
    let server_addr = "127.0.0.1:34256";
    let socket = UdpSocket::bind(server_addr).unwrap();
    println!("Server: {}", server_addr);
    loop {
        let mut buf = [0; 64];
        let (size, _) = socket.recv_from(&mut buf).unwrap();
        let filled = &mut buf[..size];
        match RequestPacket::parse(filled) {
            Ok(request_packet) => {
                println!("Request Packet: {:?}", request_packet);
                match request_packet.request_type {
                    RequestType::Login => {
                        let login_data =
                            Login::parse(request_packet.payload_length, &request_packet.payload);
                        println!("Login Data: {:?}", login_data);
                        println!("Login Request Type");
                    }
                    RequestType::HeartBeat => {
                        let heartbeat_data = Heartbeat::parse(
                            request_packet.payload_length,
                            &request_packet.payload,
                        );
                        println!("Heartbeat Data: {:?}", heartbeat_data);
                        println!("HeartBeat Request Type");
                    }
                    RequestType::Logout => {
                        let logout_data =
                            Logout::parse(request_packet.payload_length, &request_packet.payload);
                        println!("Logout Data: {:?}", logout_data);
                        println!("Logout Request Type");
                    }
                    RequestType::Coordinates => {
                        let coordinates_data = Coordinates::parse(
                            request_packet.payload_length,
                            &request_packet.payload,
                        );
                        println!("Coordinates Data: {:?}", coordinates_data);
                        println!("Coordinates Request Type");
                    }
                    _ => {
                        panic!("Invalid Request Type");
                    }
                }
            }
            Err(error) => panic!("{:?}", error),
        }
    }
    // Ok(())
}
