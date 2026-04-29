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

#[derive(Debug, Clone)]
pub struct UsbTransportConfig {
    pub vendor_id: u16,
    pub product_id: u16,
    pub interface: u8,
    pub endpoint_in: u8,
    pub endpoint_out: u8,
    pub buffer_size: usize,
    pub read_timeout_ms: u64,
    pub transact_timeout_ms: u64,
}

pub struct UsbTransport {
    id: TransportId,
    config: UsbTransportConfig,
}

impl UsbTransport {
    pub fn new(config: UsbTransportRuntimeConfig) -> io::Result<Self> {
        let parsed_config = UsbTransportConfig {
            vendor_id: parse_hex_u16(&config.vendor_id)?,
            product_id: parse_hex_u16(&config.product_id)?,
            interface: config.interface,
            endpoint_in: parse_hex_u8(&config.endpoint_in)?,
            endpoint_out: parse_hex_u8(&config.endpoint_out)?,
            buffer_size: config.buffer_size,
            read_timeout_ms: config.read_timeout_ms,
            transact_timeout_ms: config.transact_timeout_ms,
        };

        Ok(Self {
            id: config.transport_id.clone(),
            config: parsed_config,
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn synthetic_source_addr(&self) -> SocketAddr {
        USB_SYNTHETIC_SOURCE_ADDR
    }

    pub fn source_id(&self) -> String {
        format!(
            "usb://vid=0x{:04x},pid=0x{:04x},interface={}",
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

fn parse_hex_u16(value: &str) -> io::Result<u16> {
    let trimmed = value.trim();
    let hex = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
        .unwrap_or(trimmed);

    u16::from_str_radix(hex, 16).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("invalid u16 hex value '{value}': {error}"),
        )
    })
}

fn parse_hex_u8(value: &str) -> io::Result<u8> {
    let trimmed = value.trim();
    let hex = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
        .unwrap_or(trimmed);

    u8::from_str_radix(hex, 16).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("invalid u8 hex value '{value}': {error}"),
        )
    })
}
