mod config;
mod control;
mod engine;
mod instance;
mod stream;
mod transport;
mod types;

use std::net::{Ipv4Addr, SocketAddr};

use transport::udp::{UdpTransport, UdpTransportConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bind_addr: SocketAddr = "0.0.0.0:5000".parse()?;

    let config = UdpTransportConfig {
        bind_addr,
        buffer_size: 4096,
        multicast_addr: Some(Ipv4Addr::new(224, 0, 0, 5)),
        join_all_interfaces: true,
        interface_addrs: vec![],
    };

    let mut transport = UdpTransport::bind("udp-test".to_string(), config).await?;

    println!("UDP transport bound to {}", transport.config().bind_addr);
    println!("Waiting for one UDP datagram...");

    match transport.read_chunk().await? {
        Some((sender_addr, chunk)) => {
            println!(
                "Received UDP datagram: sender={}, size={} bytes",
                sender_addr,
                chunk.len()
            );
        }
        None => {
            println!("No UDP datagram received.");
        }
    }

    Ok(())
}
