//! Sensor Studio Core 메인 데몬 및 통합 실행 진입점 모듈

pub mod config;
pub mod engine;
pub mod instance;
pub mod runtime;
pub mod stream;
pub mod transport;
pub mod types;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::sync::broadcast;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::loader::load_runtime_config;
use crate::instance::{Instance, InstanceState};
use crate::runtime::extensions::EngineExtensionRegistry;
use crate::runtime::factory::{
    build_engine_extension_adapter, build_shared_engine, build_udp_transport,
};
use crate::stream::StreamPublisher;
use crate::stream::channel::ChannelRegistry;
use crate::stream::websocket::{
    WebSocketMessage, WebSocketPublisher, WebSocketServer, WebSocketServerState,
};

/// 비동기 애플리케이션 데몬 구동 및 생명주기 관리
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 날짜별 파일 기준 롤링 로거 설정
    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .max_log_files(30)
        .filename_prefix("sensor-studio-core")
        .filename_suffix("log")
        .build("logs")
        .expect("failed to create log directory or file");

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let timer = ChronoLocal::new("%Y-%m-%d %H:%M:%S%.3f".to_string());

    // 콘솔/파일 환경변수 분리 필터 구성
    let console_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| "trace".into());
    let file_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| "sensor_studio_core=info".into());

    let console_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_timer(timer.clone())
        .with_filter(console_filter);

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_timer(timer)
        .with_filter(file_filter);

    // 통합 레이어 레지스트리 초기화
    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .init();

    let mut config_path = "config/runtime.toml".to_string();
    let mut cli_ws_bind_addr: Option<SocketAddr> = None;
    let mut cli_broadcast_capacity: Option<usize> = None;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--config" | "-c" => {
                if let Some(val) = args.next() {
                    config_path = val;
                }
            }
            "--ws-addr" | "-w" => {
                if let Some(val) = args.next() {
                    cli_ws_bind_addr = Some(val.parse().expect("invalid WebSocket address format"));
                }
            }
            "--broadcast-capacity" | "-b" => {
                if let Some(val) = args.next() {
                    cli_broadcast_capacity =
                        Some(val.parse().expect("invalid broadcast capacity format"));
                }
            }
            _ => {}
        }
    }

    let runtime_config = match load_runtime_config(&config_path) {
        Ok(config) => config,
        Err(e) => {
            tracing::error!("Failed to load runtime config from {}: {}", config_path, e);
            return Err(e);
        }
    };

    if runtime_config.instances.is_empty() {
        tracing::error!("Runtime config must contain at least one instance");
        return Err("runtime config must contain at least one instance".into());
    }

    let ws_bind_addr = cli_ws_bind_addr.unwrap_or(runtime_config.server.ws_bind_addr);
    let broadcast_capacity =
        cli_broadcast_capacity.unwrap_or(runtime_config.server.broadcast_capacity);

    let (sender, _) = broadcast::channel::<WebSocketMessage>(broadcast_capacity);

    let channel_registry = Arc::new(ChannelRegistry::from_instance_configs(
        &runtime_config.instances,
    ));

    let mut extension_entries = HashMap::new();

    for instance_config in &runtime_config.instances {
        match build_engine_extension_adapter(instance_config) {
            Ok(shared) => {
                extension_entries.insert(instance_config.instance_id.clone(), shared);
            }
            Err(e) => {
                tracing::error!(
                    "Failed to build engine extension adapter for instance {}: {}",
                    instance_config.instance_id,
                    e
                );
                return Err(e);
            }
        }
    }

    let extension_registry = EngineExtensionRegistry::new(extension_entries);

    let ws_state = WebSocketServerState {
        sender: sender.clone(),
        channel_registry,
        extension_registry: extension_registry.clone(),
    };

    tokio::spawn(async move {
        if let Err(error) = WebSocketServer::serve(ws_bind_addr, ws_state).await {
            tracing::error!("WebSocket server error: {error}");
        }
    });

    tracing::info!("Sensor Studio Core daemon started (WS: {})", ws_bind_addr);

    for instance_config in runtime_config.instances {
        let sender = sender.clone();
        let extension_registry = extension_registry.clone();

        tokio::spawn(async move {
            let publish_source_id = instance_config.channel.source_id.clone();

            let shared = match extension_registry.get(&instance_config.instance_id) {
                Some(value) => value,
                None => {
                    tracing::error!(
                        "engine extension adapter not found for instance '{}'",
                        instance_config.instance_id
                    );
                    return;
                }
            };

            let transport = match build_udp_transport(&instance_config).await {
                Ok(value) => value,
                Err(error) => {
                    tracing::error!(
                        "transport setup failed for instance '{}': {error}",
                        instance_config.instance_id
                    );
                    return;
                }
            };

            let engine = match build_shared_engine(&instance_config, shared) {
                Ok(value) => value,
                Err(error) => {
                    tracing::error!(
                        "engine setup failed for instance '{}': {error}",
                        instance_config.instance_id
                    );
                    return;
                }
            };

            let mut instance =
                Instance::new(instance_config.instance_id.clone(), engine, transport);

            let mut publisher = WebSocketPublisher::new(sender, publish_source_id);

            instance.set_state(InstanceState::Running);

            const MAX_BACKOFF: std::time::Duration = std::time::Duration::from_secs(5);
            let mut backoff = std::time::Duration::from_millis(100);

            loop {
                match instance.run_once().await {
                    Ok(frames) => {
                        if instance.state == InstanceState::Error {
                            tracing::info!(
                                "Instance '{}' recovered from runtime error",
                                instance.id
                            );
                            backoff = std::time::Duration::from_millis(100);
                        }
                        instance.set_state(InstanceState::Running);

                        for frame in frames {
                            publisher.publish(frame);
                        }
                    }
                    Err(error) => {
                        instance.set_state(InstanceState::Error);
                        tracing::error!(
                            "Runtime loop error for instance '{}': {error}, retrying in {:?}",
                            instance.id,
                            backoff
                        );
                        tokio::time::sleep(backoff).await;
                        backoff = (backoff * 2).min(MAX_BACKOFF);
                    }
                }
            }
        });
    }

    tokio::signal::ctrl_c().await?;
    tracing::info!("Received Ctrl-C, gracefully shutting down Sensor Studio Core");
    Ok(())
}
