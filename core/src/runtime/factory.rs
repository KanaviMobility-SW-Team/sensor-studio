use crate::config::{InstanceRuntimeConfig, TransportRuntimeConfig};
use crate::engine::Engine;
use crate::runtime::adapter::FfiEngineAdapter;
use crate::runtime::loader::EngineLibrary;
use crate::transport::udp::{UdpTransport, UdpTransportConfig};

pub fn build_engine(
    config: &InstanceRuntimeConfig,
) -> Result<Box<dyn Engine + Send>, Box<dyn std::error::Error>> {
    let engine_config = &config.engine;
    let library = unsafe { EngineLibrary::load(&engine_config.library_path)? };
    let adapter = unsafe {
        FfiEngineAdapter::new(
            engine_config.id.clone(),
            library,
            engine_config.config_path.as_deref(),
        )?
    };

    Ok(Box::new(adapter))
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
