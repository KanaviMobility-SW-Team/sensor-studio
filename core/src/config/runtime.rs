use serde::Deserialize;
use std::collections::BTreeMap;
use std::net::{Ipv4Addr, SocketAddr};

#[derive(Clone, Debug, Deserialize)]
pub struct RuntimeConfig {
    pub instances: Vec<InstanceRuntimeConfig>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InstanceRuntimeConfig {
    pub instance_id: String,
    pub engine: EngineRuntimeConfig,
    pub transport: TransportRuntimeConfig,
    pub channel: InstanceChannelConfig,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum EngineRuntimeConfig {
    Mock {
        id: String,
    },
    External {
        id: String,
        library_path: String,
        config_path: Option<String>,
        #[serde(default)]
        settings: BTreeMap<String, String>,
    },
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TransportRuntimeConfig {
    Udp(UdpTransportRuntimeConfig),
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

#[derive(Clone, Debug, Deserialize)]
pub struct InstanceChannelConfig {
    pub channel_id: u32,
    pub source_id: String,
    pub topic: String,
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
