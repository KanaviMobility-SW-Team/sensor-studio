mod config;
mod control;
mod engine;
mod instance;
mod stream;
mod transport;
mod types;

use std::net::{Ipv4Addr, SocketAddr};

use crate::engine::mock::MockEngine;
use crate::instance::Instance;
use crate::stream::StreamPublisher;
use crate::stream::console::ConsolePublisher;
use crate::transport::udp::{UdpTransport, UdpTransportConfig};

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
    let mut publisher = ConsolePublisher::new();

    println!("Waiting for one UDP datagram...");

    let frames = instance.run_once().await?;

    println!("Processed {} frame(s)", frames.len());

    for frame in frames {
        publisher.publish(frame);
    }

    Ok(())
}
