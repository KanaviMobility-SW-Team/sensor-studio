use crate::config::{EngineRuntimeConfig, InstanceRuntimeConfig, TransportRuntimeConfig};
use crate::engine::Engine;
use crate::engine::mock::MockEngine;
use crate::runtime::adapter::FfiEngineAdapter;
use crate::runtime::loader::ExternalEngineLibrary;
use crate::transport::udp::{UdpTransport, UdpTransportConfig};

pub fn build_engine(
    config: &InstanceRuntimeConfig,
) -> Result<Box<dyn Engine + Send>, Box<dyn std::error::Error>> {
    let engine: Box<dyn Engine> = match &config.engine {
        EngineRuntimeConfig::Mock { id } => Box::new(MockEngine::new(id)),
        EngineRuntimeConfig::External {
            id,
            library_path,
            config_path,
            ..
        } => {
            let library = unsafe { ExternalEngineLibrary::load(library_path)? };
            let adapter =
                unsafe { FfiEngineAdapter::new(id.clone(), library, config_path.as_deref())? };
            Box::new(adapter)
        }
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
