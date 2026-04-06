use std::io;
use std::net::{Ipv4Addr, SocketAddr};

use bytes::Bytes;
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use socket2::{Domain, Protocol, Socket, Type};
use tokio::net::UdpSocket;

use super::TransportId;

#[derive(Clone, Debug)]
pub struct UdpTransportConfig {
    pub bind_addr: SocketAddr,
    pub buffer_size: usize,
    pub multicast_addr: Option<Ipv4Addr>,
    pub join_all_interfaces: bool,
    pub interface_addrs: Vec<Ipv4Addr>,
}

pub struct UdpTransport {
    id: TransportId,
    config: UdpTransportConfig,
    socket: UdpSocket,
    recv_buffer: Vec<u8>,
}

impl UdpTransport {
    pub async fn bind(id: TransportId, config: UdpTransportConfig) -> io::Result<Self> {
        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;

        socket.set_reuse_address(true)?;

        socket.bind(&config.bind_addr.into())?;

        if let Some(multicast_addr) = config.multicast_addr {
            let join_targets = if config.join_all_interfaces {
                Self::discover_ipv4_interfaces()?
            } else {
                config.interface_addrs.clone()
            };

            for interface_ip in join_targets {
                if interface_ip.is_loopback() {
                    continue;
                }

                let _ = socket.join_multicast_v4(&multicast_addr, &interface_ip);
            }
        }

        socket.set_nonblocking(true)?;

        let std_socket: std::net::UdpSocket = socket.into();
        let socket = UdpSocket::from_std(std_socket)?;
        let recv_buffer = vec![0u8; config.buffer_size];

        Ok(Self {
            id,
            config,
            socket,
            recv_buffer,
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn config(&self) -> &UdpTransportConfig {
        &self.config
    }

    pub async fn read_chunk(&mut self) -> io::Result<Option<(SocketAddr, Bytes)>> {
        let (size, sender_addr) = self.socket.recv_from(&mut self.recv_buffer).await?;

        if size == 0 {
            return Ok(None);
        }

        let data = Bytes::copy_from_slice(&self.recv_buffer[..size]);
        Ok(Some((sender_addr, data)))
    }

    fn discover_ipv4_interfaces() -> io::Result<Vec<Ipv4Addr>> {
        let interfaces = NetworkInterface::show()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        let mut result = Vec::new();

        for interface in interfaces {
            for addr in interface.addr {
                if let network_interface::Addr::V4(ipv4_addr) = addr {
                    result.push(ipv4_addr.ip);
                }
            }
        }

        Ok(result)
    }
}
