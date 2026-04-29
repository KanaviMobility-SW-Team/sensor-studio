//! 주요 런타임 요소 생성 팩토리 함수 모듈

use std::ffi::{CStr, c_char};
use std::sync::{Arc, Mutex};

use crate::config::{InstanceRuntimeConfig, TransportRuntimeConfig};
use crate::engine::Engine;
use crate::runtime::adapter::{FfiEngineAdapter, SharedFfiEngineAdapter};
use crate::runtime::extensions::SharedEngineExtension;
use crate::runtime::ffi::{FFI_STATUS_OK, FfiLogLevel};
use crate::runtime::loader::EngineLibrary;
use crate::transport::Transport;
use crate::transport::udp::{UdpTransport, UdpTransportConfig};
use crate::transport::usb::UsbTransport;

unsafe extern "C" fn engine_log_callback(
    level: FfiLogLevel,
    target_ptr: *const c_char,
    message_ptr: *const c_char,
) {
    if message_ptr.is_null() {
        return;
    }

    let target = if target_ptr.is_null() {
        "engine"
    } else {
        unsafe { CStr::from_ptr(target_ptr) }
            .to_str()
            .unwrap_or("engine")
    };

    let message = unsafe { CStr::from_ptr(message_ptr) }
        .to_string_lossy()
        .into_owned();

    match level {
        FfiLogLevel::Error => {
            tracing::error!("[{}]: {}", target, message)
        }
        FfiLogLevel::Warn => {
            tracing::warn!("[{}]: {}", target, message)
        }
        FfiLogLevel::Info => {
            tracing::info!("[{}]: {}", target, message)
        }
        FfiLogLevel::Debug => {
            tracing::debug!("[{}]: {}", target, message)
        }
        FfiLogLevel::Trace => {
            tracing::trace!("[{}]: {}", target, message)
        }
    }
}

/// 설정 파일 기반 FFI 엔진 어댑터 객체 초기화 및 생성
pub fn build_engine_extension_adapter(
    config: &InstanceRuntimeConfig,
) -> Result<SharedEngineExtension, Box<dyn std::error::Error>> {
    let engine_config = &config.engine;
    let library = unsafe { EngineLibrary::load(&engine_config.library_path)? };

    // Engine FFI 라이브러리에서 제공하는 버전 정보 조회 (디버깅 및 호환성 검증 목적)
    let version_cstr = unsafe {
        let version_ptr = (library.get_engine_version)();
        if version_ptr.is_null() {
            tracing::warn!(
                "Engine library '{}' returned null version string pointer",
                engine_config.library_path
            );
            CStr::from_bytes_with_nul(b"unknown\0").unwrap()
        } else {
            CStr::from_ptr(version_ptr)
        }
    };

    let version_str = version_cstr.to_string_lossy();
    tracing::info!(
        "Loaded engine library '{}', version: {}",
        engine_config.library_path,
        version_str
    );

    // logger callback 등록
    let set_logger_status =
        unsafe { (library.set_logger)(Some(engine_log_callback), FfiLogLevel::Debug) };

    if set_logger_status != FFI_STATUS_OK {
        return Err(format!(
            "failed to register engine logger for '{}': status={set_logger_status}",
            config.engine.id
        )
        .into());
    }

    let unified_config = serde_json::json!({
        "id": config.engine.id,
        "frame_id": config.channel.frame_id,
        "sensors": config.engine.sensors,
        "settings": config.engine.settings,
    });
    let config_json = unified_config.to_string();

    let adapter =
        unsafe { FfiEngineAdapter::new(config.engine.id.clone(), library, &config_json)? };

    Ok(Arc::new(Mutex::new(adapter)))
}

/// 단일 엔진 모듈에 대한 공유 어댑터 래퍼 생성
pub fn build_shared_engine(
    config: &InstanceRuntimeConfig,
    shared: SharedEngineExtension,
) -> Result<Box<dyn Engine + Send>, Box<dyn std::error::Error>> {
    Ok(Box::new(SharedFfiEngineAdapter::new(
        config.engine.id.clone(),
        shared,
    )))
}

/// 인스턴스 설정에 맞는 트랜스포트 객체 초기화
pub async fn build_transport(
    config: &InstanceRuntimeConfig,
) -> Result<Box<dyn Transport + Send>, Box<dyn std::error::Error>> {
    match &config.transport {
        TransportRuntimeConfig::Udp(transport) => {
            let transport_config = UdpTransportConfig {
                bind_addr: transport.bind_addr,
                buffer_size: transport.buffer_size,
                multicast_addr: transport.multicast_addr,
                join_all_interfaces: transport.join_all_interfaces,
                interface_addrs: transport.interface_addrs.clone(),
            };

            let transport =
                UdpTransport::bind(config.instance_id.clone(), transport_config).await?;

            Ok(Box::new(transport) as Box<dyn Transport + Send>)
        }
        TransportRuntimeConfig::Usb(transport) => {
            let transport = UsbTransport::new(transport.clone())?;

            Ok(Box::new(transport) as Box<dyn Transport + Send>)
        }
    }
}
