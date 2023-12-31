use crate::data::DataServiceImpl;
use schemas::kv::data_service_server::DataServiceServer;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tonic::transport::Server;
use tracing::info;
use wal::Storage;

mod data;

const WAL_FILE: &str = "kv.wal";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup the logger
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let storage = Arc::new(Storage::new(WAL_FILE).await?);

    // Create the services
    let data_service = DataServiceServer::new(DataServiceImpl::new(storage.clone()));

    // Start the server
    let addr = addr();
    info!("kv server running on {}", addr);

    // Start the server
    Server::builder()
        .add_service(data_service)
        .serve(addr)
        .await?;

    Ok(())
}

pub fn addr() -> SocketAddr {
    let ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    SocketAddr::new(ip, 7676)
}
