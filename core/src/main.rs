mod config;
mod control;
mod engine;
mod instance;
mod runtime;
mod stream;
mod transport;
mod types;

use std::net::SocketAddr;
use std::sync::Arc;

use tokio::sync::broadcast;

use crate::config::loader::load_runtime_config;
use crate::instance::Instance;
use crate::runtime::factory::{build_engine, build_udp_transport};
use crate::stream::StreamPublisher;
use crate::stream::channel::ChannelRegistry;
use crate::stream::websocket::{
    WebSocketMessage, WebSocketPublisher, WebSocketServer, WebSocketServerState,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws_bind_addr: SocketAddr = "0.0.0.0:8080".parse()?;

    let (sender, _) = broadcast::channel::<WebSocketMessage>(32);

    let runtime_config = load_runtime_config("src/config/runtime.toml.example")?;
    let instance_config: &&config::InstanceRuntimeConfig = &&runtime_config.instances[0];

    let publish_source_id = instance_config.channel.source_id.clone();

    let channel_registry = Arc::new(ChannelRegistry::from_instance_configs(
        &runtime_config.instances,
    ));

    let transport = build_udp_transport(instance_config).await?;
    let engine = build_engine(instance_config)?;

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
