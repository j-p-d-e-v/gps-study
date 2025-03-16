pub mod udp_client;
pub use udp_client::{CoordinatesItem, UdpClient};

#[cfg(test)]
mod tests {

    use crate::{CoordinatesItem, UdpClient};
    use gps_tracker::RequestType;
    use std::{fs::File, io::Read};

    #[tokio::test]
    async fn load_coordinates() {
        match File::options()
            .read(true)
            .open("/mnt/d/coding/study-udp/storage/sample_data_1.json")
        {
            Ok(mut file) => {
                let mut buf = String::new();
                println!("Size: {}", file.read_to_string(&mut buf).unwrap());

                let mut data: Vec<CoordinatesItem> = serde_json::from_str(&buf).unwrap();
                for _ in 0..4 {
                    for item in &data {
                        println!("LON:{}, LAT:{}", item.lon, item.lat);
                    }
                    data.reverse();
                    println!("============");
                }
                println!("Length: {}", data.len());
            }
            Err(error) => {
                assert!(false, "{:?}", error.to_string());
            }
        }
    }

    #[tokio::test]
    async fn test_client() {
        let client: Result<UdpClient, String> = UdpClient::new(
            "0.0.0.0:7082".to_string(),
            "test1".to_string(),
            "notsecurepassword".to_string(),
            "../config.toml".to_string(),
        )
        .await;
        assert!(client.is_ok(), "{:?}", client.err());
        let mut client: UdpClient = client.unwrap();
        let simulation: Result<String, String> =
            client.simulate(RequestType::Login, None, None).await;
        assert!(simulation.is_ok(), "{:?}", simulation.err());

        let hb_client_id: String = simulation.unwrap().clone();
        let client_id: String = hb_client_id.clone();
        println!("Client Id: {}", client_id);
        let handler = tokio::spawn(async move {
            let client: Result<UdpClient, String> = UdpClient::new(
                "0.0.0.0:7086".to_string(),
                "test1".to_string(),
                "notsecurepassword".to_string(),
                "../config.toml".to_string(),
            )
            .await;
            assert!(client.is_ok(), "{:?}", client.err());
            let mut client: UdpClient = client.unwrap();

            loop {
                let simulation: Result<String, String> = client
                    .simulate(RequestType::HeartBeat, Some(hb_client_id.clone()), None)
                    .await;
                assert!(simulation.is_ok(), "{:?}", simulation.err());
                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            }
        });

        let path = "/mnt/d/coding/study-udp/storage/sample_data_1.json".to_string();
        let coordinates_data = client.load_coordinates_data(path).await;
        assert!(coordinates_data.is_ok(), "{:?}", coordinates_data.err());
        let mut coordinates_data = coordinates_data.unwrap();
        for _ in 0..5 {
            for item in &coordinates_data {
                let result = client
                    .simulate(
                        RequestType::Coordinates,
                        Some(client_id.clone()),
                        Some(item.to_owned()),
                    )
                    .await;
                assert!(result.is_ok(), "{:?}", result.err());
            }
            coordinates_data.reverse();
        }
        let _ = tokio::time::timeout(std::time::Duration::from_secs(10), handler).await;
        let result = client
            .simulate(RequestType::Logout, Some(client_id), None)
            .await;
        assert!(result.is_ok(), "{:?}", result.err());
    }
}
