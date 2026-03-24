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

    let runtime_config = load_runtime_config("config/runtime.toml")?;

    if runtime_config.instances.is_empty() {
        return Err("runtime config must contain at least one instance".into());
    }

    let channel_registry = Arc::new(ChannelRegistry::from_instance_configs(
        &runtime_config.instances,
    ));

    let ws_state = WebSocketServerState {
        sender: sender.clone(),
        channel_registry,
    };

    tokio::spawn(async move {
        if let Err(error) = WebSocketServer::serve(ws_bind_addr, ws_state).await {
            eprintln!("websocket server error: {error}");
        }
    });

    println!("WebSocket server listening on {}", ws_bind_addr);

    for instance_config in runtime_config.instances {
        let sender = sender.clone();
        tokio::spawn(async move {
            let publish_source_id = instance_config.channel.source_id.clone();

            let transport = match build_udp_transport(&instance_config).await {
                Ok(value) => value,
                Err(error) => {
                    eprintln!(
                        "transport setup failed for instance '{}': {error}",
                        instance_config.instance_id
                    );
                    return;
                }
            };

            let engine = match build_engine(&instance_config) {
                Ok(value) => value,
                Err(error) => {
                    eprintln!(
                        "engine setup failed for instance '{}': {error}",
                        instance_config.instance_id
                    );
                    return;
                }
            };

            let mut instance =
                Instance::new(instance_config.instance_id.clone(), engine, transport);

            let mut publisher = WebSocketPublisher::new(sender, publish_source_id);

            println!("instance '{}' started", instance_config.instance_id);

            loop {
                match instance.run_once().await {
                    Ok(frames) => {
                        for frame in frames {
                            publisher.publish(frame);
                        }
                    }
                    Err(error) => {
                        eprintln!(
                            "runtime loop failed for instance '{}': {error}",
                            instance_config.instance_id
                        );
                        break;
                    }
                }
            }
        });
    }

    tokio::signal::ctrl_c().await?;
    Ok(())
}
