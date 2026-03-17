mod config;
mod control;
mod engine;
mod instance;
mod stream;
mod transport;
mod types;

use std::net::SocketAddr;

use crate::stream::websocket::WebSocketServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bind_addr: SocketAddr = "0.0.0.0:8080".parse()?;

    println!("Starting WebSocket server on {}", bind_addr);

    WebSocketServer::serve(bind_addr).await?;

    Ok(())
}
