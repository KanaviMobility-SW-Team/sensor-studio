use std::net::{Ipv4Addr, SocketAddr};

#[derive(Clone, Debug)]
pub struct InstanceRuntimeConfig {
    pub instance_id: String,
    pub engine: EngineRuntimeConfig,
    pub transport: TransportRuntimeConfig,
    pub channel: InstanceChannelConfig,
}

#[derive(Clone, Debug)]
pub struct EngineRuntimeConfig {
    pub kind: EngineKindConfig,
    pub id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EngineKindConfig {
    Mock,
}

#[derive(Clone, Debug)]
pub enum TransportRuntimeConfig {
    Udp(UdpTransportRuntimeConfig),
}

#[derive(Clone, Debug)]
pub struct UdpTransportRuntimeConfig {
    pub bind_addr: SocketAddr,
    pub buffer_size: usize,
    pub multicast_addr: Option<Ipv4Addr>,
    pub join_all_interfaces: bool,
    pub interface_addrs: Vec<Ipv4Addr>,
}

#[derive(Clone, Debug)]
pub struct InstanceChannelConfig {
    pub channel_id: u32,
    pub source_id: String,
    pub topic: String,
    pub schema: ChannelSchemaConfig,
    pub encoder: ChannelEncoderConfig,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChannelSchemaConfig {
    PointCloud,
    Status,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChannelEncoderConfig {
    Json,
}
