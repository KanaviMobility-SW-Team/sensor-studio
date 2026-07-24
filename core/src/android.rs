#![allow(non_snake_case)]

use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use std::thread::{self, JoinHandle};

use android_logger::Config;
use jni::EnvUnowned;
use jni::objects::{JObject, JString};
use log::{LevelFilter, error, info, warn};
use tokio_util::sync::CancellationToken;

/// Android에서 실행 중인 Rust worker 정보
struct AndroidWorker {
    shutdown_token: CancellationToken,
    thread: JoinHandle<()>,
}

/// 현재 실행 중인 worker를 전역으로 관리
static WORKER: OnceLock<Mutex<Option<AndroidWorker>>> = OnceLock::new();

fn worker() -> &'static Mutex<Option<AndroidWorker>> {
    WORKER.get_or_init(|| Mutex::new(None))
}

/// Android Logcat logger 초기화
///
/// 여러 번 호출되어도 실제 초기화는 한 번만 수행됨.
fn init_logger() {
    android_logger::init_once(
        Config::default()
            .with_tag("SensorStudioCore")
            .with_max_level(LevelFilter::Info),
    );
}

/// Sensor Studio Core 실행
///
/// `files_dir`은 Kotlin의 `filesDir.absolutePath` 값이다.
///
/// 예:
/// /data/user/0/com.example.sensorstudiocore/files
pub fn start(files_dir: PathBuf) {
    init_logger();

    info!("Rust start() called");
    info!("App files directory: {}", files_dir.display());

    let mut current = match worker().lock() {
        Ok(current) => current,
        Err(error) => {
            error!("Failed to lock Android worker: {error}");
            return;
        }
    };

    // 이미 실행 중이면 중복 실행하지 않음
    if let Some(existing) = current.as_ref() {
        if !existing.thread.is_finished() {
            warn!("Rust worker is already running");
            return;
        }
    }

    // 이전 worker가 종료된 상태라면 join 처리
    if let Some(finished) = current.take() {
        match finished.thread.join() {
            Ok(()) => {
                info!("Previous Rust worker joined");
            }
            Err(_) => {
                error!("Previous Rust worker thread panicked");
            }
        }
    }

    let shutdown_token = CancellationToken::new();
    let run_shutdown_token = shutdown_token.clone();

    let thread = match thread::Builder::new()
        .name("sensor-studio-core".to_string())
        .spawn(move || {
            info!("Rust worker thread started");

            // Android 앱 전용 디렉터리를 현재 작업 디렉터리로 사용
            if let Err(error) = std::env::set_current_dir(&files_dir) {
                error!(
                    "Failed to change current directory to {}: {error}",
                    files_dir.display()
                );
                return;
            }

            let current_dir = match std::env::current_dir() {
                Ok(path) => path,
                Err(error) => {
                    error!("Failed to read current directory: {error}");
                    return;
                }
            };

            info!("Current directory: {}", current_dir.display());

            // assets에서 Kotlin이 복사한 설정 파일
            let config_path = files_dir.join("config/runtime.toml");

            info!("Runtime config path: {}", config_path.display());
            info!("Runtime config exists: {}", config_path.exists());

            if !config_path.exists() {
                error!("Runtime config does not exist: {}", config_path.display());
                return;
            }

            // crate::run() 내부의 .build("logs")가 사용할 디렉터리
            let logs_dir = files_dir.join("logs");

            if let Err(error) = std::fs::create_dir_all(&logs_dir) {
                error!(
                    "Failed to create log directory {}: {error}",
                    logs_dir.display()
                );
                return;
            }

            info!("Log directory prepared: {}", logs_dir.display());

            let runtime = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .thread_name("sensor-studio-tokio")
                .build()
                .expect("failed to create Tokio runtime");

            runtime.block_on(async move {
                match crate::run_with_shutdown(run_shutdown_token).await {
                    Ok(()) => info!("Rust core stopped normally"),
                    Err(error) => error!("Rust core failed: {error}"),
                }
            });

            info!("Rust worker thread finished");
        }) {
        Ok(thread) => thread,

        Err(error) => {
            error!("Failed to spawn Rust worker thread: {error}");
            return;
        }
    };

    *current = Some(AndroidWorker {
        shutdown_token,
        thread,
    });

    info!("Rust worker thread spawned");
}

/// Sensor Studio Core 종료
pub fn stop() {
    init_logger();

    info!("Rust stop() called");

    let current = match worker().lock() {
        Ok(current) => current,
        Err(error) => {
            error!("Failed to lock Android worker: {error}");
            return;
        }
    };

    let Some(android_worker) = current.as_ref() else {
        warn!("Rust worker is not running");
        return;
    };

    info!("Requesting graceful Rust shutdown");
    android_worker.shutdown_token.cancel();
}

/// Kotlin:
///
/// private external fun nativeStart(filesDir: String)
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_example_sensorstudiocore_RustForegroundService_nativeStart<
    'caller,
>(
    mut unowned_env: EnvUnowned<'caller>,
    _service: JObject<'caller>,
    files_dir: JString<'caller>,
) {
    init_logger();

    unowned_env
        .with_env(|_env| -> Result<(), jni::errors::Error> {
            let files_dir: String = files_dir.to_string();

            info!("Received filesDir from Kotlin: {files_dir}");

            start(PathBuf::from(files_dir));

            Ok(())
        })
        .resolve::<jni::errors::LogErrorAndDefault>();
}

/// Kotlin:
///
/// private external fun nativeStop()
#[unsafe(no_mangle)]
pub extern "system" fn Java_com_example_sensorstudiocore_RustForegroundService_nativeStop<
    'caller,
>(
    mut unowned_env: EnvUnowned<'caller>,
    _service: JObject<'caller>,
) {
    init_logger();

    unowned_env
        .with_env(|_env| -> Result<(), jni::errors::Error> {
            stop();

            Ok(())
        })
        .resolve::<jni::errors::LogErrorAndDefault>();
}
