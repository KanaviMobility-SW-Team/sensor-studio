mod config;
mod control;
mod engine;
mod instance;
mod stream;
mod transport;
mod types;

use std::net::{Ipv4Addr, SocketAddr};

use engine::mock::MockEngine;
use instance::Instance;
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

    let transport = UdpTransport::bind("udp-test".to_string(), config).await?;
    let engine = Box::new(MockEngine::new("mock-engine"));

    let mut instance = Instance::new("instance-1", engine, transport);

    println!("Waiting for one UDP datagram...");

    let frames = instance.run_once().await?;

    println!("Processed {} frame(s)", frames.len());

    for (index, frame) in frames.iter().enumerate() {
        println!(
            "frame[{index}]: frame_id={}, points={}, data_size={} bytes",
            frame.frame_id,
            frame.point_count(),
            frame.data.len()
        );
    }

    Ok(())
}
