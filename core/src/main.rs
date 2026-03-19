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
    TransportRuntimeConfig, UdpTransportRuntimeConfig,
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

    let (sender, _) = broadcast::channel::<WebSocketMessage>(32);

    let instance_configs = vec![InstanceRuntimeConfig {
        instance_id: "instance-1".to_string(),
        transport: TransportRuntimeConfig::Udp(UdpTransportRuntimeConfig {
            bind_addr: "0.0.0.0:5000".parse()?,
            buffer_size: 4096,
            multicast_addr: Some(Ipv4Addr::new(224, 0, 0, 5)),
            join_all_interfaces: true,
            interface_addrs: vec![],
        }),
        channel: InstanceChannelConfig {
            channel_id: 1,
            source_id: "mock_sensor".to_string(),
            topic: "/pointcloud/mock".to_string(),
            schema: ChannelSchemaConfig::PointCloud,
            encoder: ChannelEncoderConfig::Json,
        },
    }];

    let instance_config = &instance_configs[0];

    let publish_source_id = instance_config.channel.source_id.clone();

    let channel_registry = Arc::new(ChannelRegistry::from_instance_configs(&instance_configs));

    let transport = match &instance_config.transport {
        TransportRuntimeConfig::Udp(config) => {
            let transport_config = UdpTransportConfig {
                bind_addr: config.bind_addr,
                buffer_size: config.buffer_size,
                multicast_addr: config.multicast_addr,
                join_all_interfaces: config.join_all_interfaces,
                interface_addrs: config.interface_addrs.clone(),
            };

            UdpTransport::bind("udp-test".to_string(), transport_config).await?
        }
    };

    let engine = Box::new(MockEngine::new("mock-engine"));
    let mut instance = Instance::new(instance_config.instance_id.clone(), engine, transport);

    let mut publisher = WebSocketPublisher::new(sender.clone(), publish_source_id);

    let ws_state = WebSocketServerState {
        sender: sender,
        channel_registry,
    };

    tokio::spawn(async move {
        if let Err(error) = WebSocketServer::serve(ws_bind_addr, ws_state).await {
            eprintln!("websocket server error: {error}");
        }
    });

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
