//! USB 전송 계층 구현
//!
//! 현재 단계에서는 USB transport의 구조만 정의하고,
//! 실제 USB 장치 open/read/write 처리는 이후 단계에서 구현한다.

use std::io;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use crate::config::UsbTransportRuntimeConfig;
use crate::transport::{
    Transport, TransportChunk, TransportFuture, TransportId, TransportKind, TransportRequest,
};

/// USB 장치를 기존 Engine FFI의 sender_addr 구조에 맞추기 위한 synthetic address
const USB_SYNTHETIC_SOURCE_ADDR: SocketAddr =
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0));

pub struct UsbTransport {
    id: TransportId,
    config: UsbTransportRuntimeConfig,
}

impl UsbTransport {
    pub fn new(config: UsbTransportRuntimeConfig) -> Self {
        Self {
            id: config.transport_id.clone(),
            config,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn synthetic_source_addr(&self) -> SocketAddr {
        USB_SYNTHETIC_SOURCE_ADDR
    }

    pub fn source_id(&self) -> String {
        format!(
            "usb://vid={},pid={},interface={}",
            self.config.vendor_id, self.config.product_id, self.config.interface
        )
    }
}

impl Transport for UsbTransport {
    fn id(&self) -> &str {
        self.id()
    }

    fn kind(&self) -> TransportKind {
        TransportKind::Usb
    }

    fn read_chunk(&mut self) -> TransportFuture<'_, Option<TransportChunk>> {
        Box::pin(async move {
            Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "USB transport read is not implemented yet",
            ))
        })
    }

    fn transact_chunk(
        &mut self,
        _request: TransportRequest,
    ) -> TransportFuture<'_, Vec<TransportChunk>> {
        Box::pin(async move {
            Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "USB transport transact is not implemented yet",
            ))
        })
    }
}
