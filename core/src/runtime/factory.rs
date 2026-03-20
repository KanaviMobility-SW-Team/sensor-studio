use crate::config::{EngineKindConfig, InstanceRuntimeConfig, TransportRuntimeConfig};
use crate::engine::Engine;
use crate::engine::mock::MockEngine;
use crate::transport::udp::{UdpTransport, UdpTransportConfig};

pub fn build_engine(
    config: &InstanceRuntimeConfig,
) -> Result<Box<dyn Engine>, Box<dyn std::error::Error>> {
    let engine: Box<dyn Engine> = match &config.engine.kind {
        EngineKindConfig::Mock => Box::new(MockEngine::new(&config.engine.id)),
    };

    Ok(engine)
}

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
