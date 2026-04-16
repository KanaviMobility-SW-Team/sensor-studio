//! 주요 런타임 요소 생성 팩토리 함수 모듈

use std::sync::{Arc, Mutex};

use crate::config::{InstanceRuntimeConfig, TransportRuntimeConfig};
use crate::engine::Engine;
use crate::runtime::adapter::{FfiEngineAdapter, SharedFfiEngineAdapter};
use crate::runtime::extensions::SharedEngineExtension;
use crate::runtime::loader::EngineLibrary;
use crate::transport::udp::{UdpTransport, UdpTransportConfig};

/// 설정 파일 기반 FFI 엔진 어댑터 객체 초기화 및 생성
pub fn build_engine_extension_adapter(
    config: &InstanceRuntimeConfig,
) -> Result<SharedEngineExtension, Box<dyn std::error::Error>> {
    let engine_config = &config.engine;
    let library = unsafe { EngineLibrary::load(&engine_config.library_path)? };

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

/// UDP 트랜스포트 객체 초기화 및 네트워크 바인딩
pub async fn build_udp_transport(
    config: &InstanceRuntimeConfig,
) -> Result<UdpTransport, Box<dyn std::error::Error>> {
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

            Ok(transport)
        }
    }
}
