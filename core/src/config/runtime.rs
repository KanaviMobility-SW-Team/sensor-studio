use std::collections::{BTreeMap, HashSet};
use std::net::{Ipv4Addr, SocketAddr};
use std::path::Path;

use serde::{Deserialize, Serialize};

/// 코어 시스템 전체 런타임 설정
#[derive(Clone, Debug, Deserialize)]
pub struct RuntimeConfig {
    #[serde(default)]
    pub server: ServerConfig,
    pub instances: Vec<InstanceRuntimeConfig>,
}

impl RuntimeConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.instances.is_empty() {
            return Err("Runtime config must have at least one instance defined.".into());
        }

        let mut instance_ids = HashSet::new();

        for (index, instance) in self.instances.iter().enumerate() {
            // 중복 인스턴스 ID 검증
            if !instance_ids.insert(&instance.instance_id) {
                return Err(format!(
                    "Duplicate instance_id '{}' found at index {}.",
                    instance.instance_id, index
                ));
            }

            // 엔진 라이브러리 파일 경로 존재 여부 검증
            if !Path::new(&instance.engine.library_path).exists() {
                return Err(format!(
                    "Engine library path does not exist for instance '{}': {}",
                    instance.instance_id, instance.engine.library_path
                ));
            }

            // 토픽 이름 검증 (빈 문자열이 아니어야 함)
            if instance.channel.topic.trim().is_empty() {
                return Err(format!(
                    "Channel topic must not be empty for instance '{}'.",
                    instance.instance_id
                ));
            }

            // 트랜스포트 설정 검증
            match &instance.transport {
                TransportRuntimeConfig::Udp(udp) => {
                    // 버퍼 사이즈 검증
                    if udp.buffer_size == 0 {
                        return Err(format!(
                            "UDP buffer_size must be greater than 0 for instance '{}'.",
                            instance.instance_id
                        ));
                    }
                }
                TransportRuntimeConfig::Usb(_) => {}
            }
        }

        Ok(())
    }
}

/// 개별 센서 인스턴스 구동 설정
#[derive(Clone, Debug, Deserialize)]
pub struct InstanceRuntimeConfig {
    pub instance_id: String,
    pub engine: EngineRuntimeConfig,
    pub transport: TransportRuntimeConfig,
    pub channel: InstanceChannelConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EngineSensorConfig {
    pub ip: String,
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
    #[serde(default)]
    pub z: f32,
    #[serde(default)]
    pub roll: f32,
    #[serde(default)]
    pub pitch: f32,
    #[serde(default)]
    pub yaw: f32,
}

/// (FFI) 플러그인 로드 및 동적 주입용 데이터
#[derive(Clone, Debug, Deserialize)]
pub struct EngineRuntimeConfig {
    pub id: String,
    pub library_path: String,

    #[serde(default)]
    pub settings: BTreeMap<String, String>,

    #[serde(default)]
    pub sensors: Vec<EngineSensorConfig>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TransportRuntimeConfig {
    Udp(UdpTransportRuntimeConfig),
    Usb(UsbTransportRuntimeConfig),
}

#[derive(Clone, Debug, Deserialize)]
pub struct UdpTransportRuntimeConfig {
    pub bind_addr: SocketAddr,
    pub buffer_size: usize,
    pub multicast_addr: Option<Ipv4Addr>,
    pub join_all_interfaces: bool,
    #[serde(default)]
    pub interface_addrs: Vec<Ipv4Addr>,
}

fn default_usb_buffer_size() -> usize {
    512 * 1024 // 524288
}

fn default_usb_read_timeout_ms() -> u64 {
    1000
}

fn default_usb_transact_timeout_ms() -> u64 {
    300
}

#[derive(Debug, Clone, Deserialize)]
pub struct UsbTransportRuntimeConfig {
    pub transport_id: String,

    pub vendor_id: String,
    pub product_id: String,

    pub interface: u8,
    pub endpoint_in: String,
    pub endpoint_out: String,

    #[serde(default = "default_usb_buffer_size")]
    pub buffer_size: usize,

    #[serde(default = "default_usb_read_timeout_ms")]
    pub read_timeout_ms: u64,

    #[serde(default = "default_usb_transact_timeout_ms")]
    pub transact_timeout_ms: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InstanceChannelConfig {
    pub channel_id: u32,
    pub source_id: String,
    pub topic: String,
    pub frame_id: String,
    pub schema: ChannelSchemaConfig,
    pub encoder: ChannelEncoderConfig,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChannelSchemaConfig {
    PointCloud,
    Status,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChannelEncoderConfig {
    Json,
}

/// Foxglove WebSocket 및 전역 통신 설정
#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    pub ws_bind_addr: SocketAddr,
    pub broadcast_capacity: usize,
}

// 기본값 구현
impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            ws_bind_addr: "0.0.0.0:8080".parse().unwrap(),
            broadcast_capacity: 32,
        }
    }
}
