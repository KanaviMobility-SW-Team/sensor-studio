//! UDP 기반 데이터 네트워크 수신 계층 모듈

use std::io;
use std::net::{Ipv4Addr, SocketAddr};

use bytes::Bytes;
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use socket2::{Domain, Protocol, Socket, Type};
use tokio::net::UdpSocket;

use crate::transport::{
    Transport, TransportChunk, TransportFuture, TransportKind, TransportRequest,
};

use super::TransportId;

/// UDP 소켓 생성을 위한 구성요소 구조체
#[derive(Clone, Debug)]
pub struct UdpTransportConfig {
    pub bind_addr: SocketAddr,
    pub buffer_size: usize,
    pub multicast_addr: Option<Ipv4Addr>,
    pub join_all_interfaces: bool,
    pub interface_addrs: Vec<Ipv4Addr>,
}

/// 비동기 멀티캐스트 및 데이터그램 패킷 수신 구조체
pub struct UdpTransport {
    id: TransportId,
    config: UdpTransportConfig,
    socket: UdpSocket,
    recv_buffer: Vec<u8>,
}

impl UdpTransport {
    /// UDP 소켓 구조체 생성 및 네트워크 바인딩
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

                if let Err(e) = socket.join_multicast_v4(&multicast_addr, &interface_ip) {
                    tracing::warn!(
                        "Failed to join multicast {} on interface {}: {}",
                        multicast_addr,
                        interface_ip,
                        e
                    );
                }
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

    /// 데이터 패킷 비동기 수신
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

impl Transport for UdpTransport {
    fn id(&self) -> &str {
        self.id()
    }

    fn kind(&self) -> TransportKind {
        TransportKind::Udp
    }

    fn read_chunk(&mut self) -> TransportFuture<'_, Option<TransportChunk>> {
        Box::pin(async move {
            let Some((source_addr, data)) = UdpTransport::read_chunk(self).await? else {
                return Ok(None);
            };

            Ok(Some(TransportChunk {
                source_addr,
                source_id: format!("udp://{}", source_addr),
                data,
            }))
        })
    }

    fn transact_chunk(
        &mut self,
        _request: TransportRequest,
    ) -> TransportFuture<'_, Option<TransportChunk>> {
        Box::pin(async move {
            Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "UDP transport transact is not supported yet",
            ))
        })
    }
}
