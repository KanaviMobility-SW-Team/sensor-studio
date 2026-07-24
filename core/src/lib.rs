//! Sensor Studio Core 메인 데몬 및 통합 실행 진입점 모듈

pub mod config;
pub mod engine;
pub mod instance;
pub mod runtime;
pub mod stream;
pub mod transport;
pub mod types;

use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex, OnceLock};

use tokio::sync::broadcast;
use tokio_util::sync::CancellationToken;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt};

use crate::config::loader::load_runtime_config;
use crate::instance::{Instance, InstanceState};
use crate::runtime::extensions::EngineExtensionRegistry;
use crate::runtime::factory::{
    build_engine_extension_adapter, build_shared_engine, build_transport,
};
use crate::stream::StreamPublisher;
use crate::stream::channel::ChannelRegistry;
use crate::stream::websocket::{
    WebSocketMessage, WebSocketPublisher, WebSocketServer, WebSocketServerState,
};

#[cfg(target_os = "android")]
mod android;

/// 비동기 파일 로거의 WorkerGuard를 프로세스 종료까지 유지한다.
///
/// tracing subscriber는 프로세스당 한 번만 등록할 수 있으므로,
/// Android Service를 중지했다 다시 시작하더라도 재등록하지 않는다.
static TRACING_GUARD: OnceLock<Mutex<Option<WorkerGuard>>> = OnceLock::new();

fn tracing_guard() -> &'static Mutex<Option<WorkerGuard>> {
    TRACING_GUARD.get_or_init(|| Mutex::new(None))
}

/// tracing subscriber 초기화
///
/// 데스크톱과 Android 모두 최초 실행 시 한 번만 초기화된다.
fn initialize_tracing() -> Result<(), Box<dyn Error>> {
    let mut guard_slot = tracing_guard()
        .lock()
        .map_err(|_| std::io::Error::other("tracing guard lock poisoned"))?;

    // 이미 초기화된 경우 재등록하지 않는다.
    if guard_slot.is_some() {
        return Ok(());
    }

    // 날짜별 파일 기준 롤링 로거 설정
    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .max_log_files(30)
        .filename_prefix("sensor-studio-core")
        .filename_suffix("log")
        .build("logs")?;

    let (non_blocking, worker_guard) = tracing_appender::non_blocking(file_appender);

    let timer = ChronoLocal::new("%Y-%m-%d %H:%M:%S%.3f".to_string());

    // 콘솔/파일 환경변수 분리 필터 구성
    let console_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| "debug".into());

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

    let subscriber = tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer);

    tracing::subscriber::set_global_default(subscriber)?;

    // WorkerGuard가 drop되면 파일 로깅 worker가 종료되므로
    // static 영역에 보관한다.
    *guard_slot = Some(worker_guard);

    Ok(())
}

/// 데스크톱 실행 진입점
///
/// Ctrl-C를 CancellationToken으로 변환한 뒤 공통 실행 함수를 호출한다.
pub async fn run() -> Result<(), Box<dyn Error>> {
    let shutdown_token = CancellationToken::new();
    let signal_token = shutdown_token.clone();

    let signal_handle = tokio::spawn(async move {
        match tokio::signal::ctrl_c().await {
            Ok(()) => {
                tracing::info!("Received Ctrl-C, requesting Sensor Studio Core shutdown");

                signal_token.cancel();
            }

            Err(error) => {
                tracing::error!("Failed to listen for Ctrl-C signal: {error}");

                // Ctrl-C 감시 자체가 실패해도 종료가 불가능한 상태로
                // 남지 않도록 종료를 요청한다.
                signal_token.cancel();
            }
        }
    });

    let result = run_with_shutdown(shutdown_token).await;

    // run_with_shutdown이 Ctrl-C가 아닌 오류로 먼저 종료된 경우
    // signal task가 계속 남지 않도록 정리한다.
    signal_handle.abort();
    let _ = signal_handle.await;

    result
}

/// Sensor Studio Core 공통 실행 함수
///
/// Android에서는 nativeStop()이 이 함수에 전달된 토큰을 cancel한다.
/// 데스크톱에서는 Ctrl-C가 토큰을 cancel한다.
pub async fn run_with_shutdown(shutdown_token: CancellationToken) -> Result<(), Box<dyn Error>> {
    initialize_tracing()?;

    let mut config_path = "config/runtime.toml".to_string();
    let mut cli_ws_bind_addr: Option<SocketAddr> = None;
    let mut cli_broadcast_capacity: Option<usize> = None;

    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--config" | "-c" => {
                if let Some(value) = args.next() {
                    config_path = value;
                }
            }

            "--ws-addr" | "-w" => {
                if let Some(value) = args.next() {
                    cli_ws_bind_addr =
                        Some(value.parse().expect("invalid WebSocket address format"));
                }
            }

            "--broadcast-capacity" | "-b" => {
                if let Some(value) = args.next() {
                    cli_broadcast_capacity =
                        Some(value.parse().expect("invalid broadcast capacity format"));
                }
            }

            _ => {}
        }
    }

    let runtime_config = match load_runtime_config(&config_path) {
        Ok(config) => config,

        Err(error) => {
            tracing::error!(
                "Failed to load runtime config from {}: {}",
                config_path,
                error
            );

            return Err(error);
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

            Err(error) => {
                tracing::error!(
                    "Failed to build engine extension adapter for instance {}: {}",
                    instance_config.instance_id,
                    error
                );

                return Err(error);
            }
        }
    }

    let extension_registry = EngineExtensionRegistry::new(extension_entries);

    let ws_state = WebSocketServerState {
        sender: sender.clone(),
        channel_registry,
        extension_registry: extension_registry.clone(),
    };

    // 종료 시 abort할 수 있도록 WebSocket JoinHandle을 보관한다.
    let websocket_handle = tokio::spawn(async move {
        if let Err(error) = WebSocketServer::serve(ws_bind_addr, ws_state).await {
            tracing::error!("WebSocket server error: {error}");
        }
    });

    tracing::info!("Sensor Studio Core daemon started (WS: {})", ws_bind_addr);

    // 외부 종료 신호와 인스턴스 종료 토큰을 분리한다.
    //
    // 외부 토큰:
    // - Android nativeStop()
    // - 데스크톱 Ctrl-C
    //
    // 내부 토큰:
    // - 각 Instance 실행 루프 종료
    let instance_token = CancellationToken::new();
    let mut instance_handles = Vec::new();

    for instance_config in runtime_config.instances {
        let sender = sender.clone();
        let extension_registry = extension_registry.clone();
        let token = instance_token.clone();

        let handle = tokio::spawn(async move {
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

            let transport = match build_transport(&instance_config).await {
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

            'runtime_loop: loop {
                tokio::select! {
                    _ = token.cancelled() => {
                        tracing::info!(
                            "Instance '{}' received shutdown signal",
                            instance.id
                        );

                        break 'runtime_loop;
                    }

                    result = instance.run_once() => {
                        match result {
                            Ok(frames) => {
                                if instance.state == InstanceState::Error {
                                    tracing::info!(
                                        "Instance '{}' recovered from runtime error",
                                        instance.id
                                    );

                                    backoff =
                                        std::time::Duration::from_millis(100);
                                }

                                instance.set_state(
                                    InstanceState::Running
                                );

                                for frame in frames {
                                    publisher.publish(frame);
                                }
                            }

                            Err(error) => {
                                instance.set_state(
                                    InstanceState::Error
                                );

                                tracing::error!(
                                    "Runtime loop error for instance '{}': {error}, retrying in {:?}",
                                    instance.id,
                                    backoff
                                );

                                // backoff 대기 중에도 종료 신호에 즉시 반응한다.
                                tokio::select! {
                                    _ = token.cancelled() => {
                                        tracing::info!(
                                            "Instance '{}' received shutdown signal during retry backoff",
                                            instance.id
                                        );

                                        break 'runtime_loop;
                                    }

                                    _ = tokio::time::sleep(backoff) => {}
                                }

                                backoff =
                                    (backoff * 2).min(MAX_BACKOFF);
                            }
                        }
                    }
                }
            }

            tracing::info!("Shutting down instance '{}'", instance.id);

            instance.shutdown().await;

            tracing::info!("Instance '{}' shutdown completed", instance.id);
        });

        instance_handles.push(handle);
    }

    // Android nativeStop() 또는 데스크톱 Ctrl-C 대기
    shutdown_token.cancelled().await;

    tracing::info!("Shutdown requested, gracefully shutting down Sensor Studio Core");

    // 각 Instance 루프에 종료 요청
    instance_token.cancel();

    // 신규 WebSocket 연결 및 기존 서버 task 종료
    websocket_handle.abort();

    // Instance의 shutdown() 완료 대기
    for handle in instance_handles {
        match handle.await {
            Ok(()) => {}

            Err(error) if error.is_cancelled() => {
                tracing::warn!("Instance task was cancelled during shutdown");
            }

            Err(error) => {
                tracing::error!("Instance task failed during shutdown: {error}");
            }
        }
    }

    match websocket_handle.await {
        Ok(()) => {
            tracing::info!("WebSocket server stopped normally");
        }

        Err(error) if error.is_cancelled() => {
            tracing::info!("WebSocket server task cancelled");
        }

        Err(error) => {
            tracing::error!("WebSocket server task failed during shutdown: {error}");
        }
    }

    tracing::info!("Sensor Studio Core shutdown completed");

    Ok(())
}
