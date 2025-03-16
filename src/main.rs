use gps_tracker::udp_server::UdpServer;

#[tokio::main]
async fn main() -> Result<(), String> {
    UdpServer::launch().await?;
    Ok(())
}
