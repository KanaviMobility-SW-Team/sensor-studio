mod config;
mod control;
mod engine;
mod instance;
mod stream;
mod transport;
mod types;

use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;

use tokio::sync::broadcast;

use crate::config::{
    ChannelEncoderConfig, ChannelSchemaConfig, InstanceChannelConfig, InstanceRuntimeConfig,
};
use crate::engine::mock::MockEngine;
use crate::instance::Instance;
use crate::stream::StreamPublisher;
use crate::stream::channel::ChannelRegistry;
use crate::stream::websocket::{
    WebSocketMessage, WebSocketPublisher, WebSocketServer, WebSocketServerState,
};
use crate::transport::udp::{UdpTransport, UdpTransportConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws_bind_addr: SocketAddr = "0.0.0.0:8080".parse()?;
    let udp_bind_addr: SocketAddr = "0.0.0.0:5000".parse()?;

    let (sender, _) = broadcast::channel::<WebSocketMessage>(32);

    let instance_configs = vec![InstanceRuntimeConfig {
        instance_id: "instance-1".to_string(),
        channel: InstanceChannelConfig {
            channel_id: 1,
            source_id: "mock_sensor".to_string(),
            topic: "/pointcloud/mock".to_string(),
            schema: ChannelSchemaConfig::PointCloud,
            encoder: ChannelEncoderConfig::Json,
        },
    }];

    let publish_source_id = instance_configs[0].channel.source_id.clone();

    let channel_registry = Arc::new(ChannelRegistry::from_instance_configs(&instance_configs));

    let ws_state = WebSocketServerState {
        sender: sender.clone(),
        channel_registry,
    };

    let mut publisher = WebSocketPublisher::new(sender, publish_source_id);

    tokio::spawn(async move {
        if let Err(error) = WebSocketServer::serve(ws_bind_addr, ws_state).await {
            eprintln!("websocket server error: {error}");
        }
    });

    let config = UdpTransportConfig {
        bind_addr: udp_bind_addr,
        buffer_size: 4096,
        multicast_addr: Some(Ipv4Addr::new(224, 0, 0, 5)),
        join_all_interfaces: true,
        interface_addrs: vec![],
    };

    let transport = UdpTransport::bind("udp-test".to_string(), config).await?;
    let engine = Box::new(MockEngine::new("mock-engine"));

    let mut instance = Instance::new(instance_configs[0].instance_id.clone(), engine, transport);

    println!("Waiting for one UDP datagram...");
    println!("WebSocket server listening on {}", ws_bind_addr);

    loop {
        let frames = instance.run_once().await?;

        for frame in frames {
            publisher.publish(frame);
        }
    }

    Ok(())
}
