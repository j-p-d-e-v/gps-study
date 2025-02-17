use gps_tracker::udp_server::UdpServer;

#[tokio::main]
async fn main() -> Result<(), String> {
    // Get input
    let mut service_input: String = String::new();

    loop {
        // Main Services
        println!("Choose Services:");
        println!("1. Payload Generator");
        println!("2. Launch UDP Server");
        println!("3. Launch UDP Client(For Simulation) - Soon!");
        if let Err(error) = std::io::stdin().read_line(&mut service_input) {
            eprintln!("Input Error: {:?}", error);
        } else {
            break;
        }
    }
    match service_input.trim().parse::<u8>() {
        Ok(input) => {
            match input {
                1 => {
                    println!("Choose Generator");
                    let mut payload_generator: String = String::new();
                    loop {
                        println!("1. Login Payload");
                        println!("2. Heartbeat Payload");
                        println!("3. Coordinates Payload");
                        if let Err(error) = std::io::stdin().read_line(&mut payload_generator) {
                            eprintln!("Input Error: {:?}", error);
                        } else {
                            break;
                        }
                    }
                    match payload_generator.trim().parse::<u8>() {
                        Ok(input) => match input {
                            1 => println!("This is login payload generator"),
                            2 => println!("This is heartbeat payload generator"),
                            3 => println!("This is coordinates payload generator"),
                            _ => panic!("unable to determine generator"),
                        },
                        Err(error) => {
                            panic!("{:?}", error);
                        }
                    }
                }
                2 => {
                    // Launch UDP Server
                    UdpServer::launch().await?;
                }
                _ => {
                    panic!("unable to determine service");
                }
            }
        }
        Err(error) => {
            panic!("{:?}", error.to_string())
        }
    }
    Ok(())
}
