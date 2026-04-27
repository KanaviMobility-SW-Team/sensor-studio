//! 시스템 전역 설정(runtime.toml) 파싱 및 데이터 모델 모듈

pub mod loader;
pub mod runtime;

pub use crate::config::loader::load_runtime_config;
pub use crate::config::runtime::{
    ChannelEncoderConfig, ChannelSchemaConfig, EngineRuntimeConfig, InstanceChannelConfig,
    InstanceRuntimeConfig, RuntimeConfig, TransportRuntimeConfig, UdpTransportRuntimeConfig,
    UsbTransportRuntimeConfig,
};
