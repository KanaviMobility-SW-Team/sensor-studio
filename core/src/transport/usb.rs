//! USB 전송 계층 구현
//!
//! 현재 단계에서는 USB transport의 구조만 정의하고,
//! 실제 USB 장치 open/read/write 처리는 이후 단계에서 구현한다.

use std::io;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::time::Duration;

use bytes::Bytes;
use rusb::{DeviceHandle, GlobalContext};

use crate::config::UsbTransportRuntimeConfig;
use crate::transport::{
    Transport, TransportChunk, TransportFuture, TransportId, TransportKind, TransportRequest,
    TransportResponseMode,
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
    handle: DeviceHandle<GlobalContext>,
    read_timeout: Duration,
    transact_timeout: Duration,
    buffer: Vec<u8>,
}

impl UsbTransport {
    pub fn new(id: TransportId, config: UsbTransportRuntimeConfig) -> io::Result<Self> {
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

        let mut handle =
            rusb::open_device_with_vid_pid(parsed_config.vendor_id, parsed_config.product_id)
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::NotFound,
                        format!(
                            "USB device not found: vid=0x{:04x}, pid=0x{:04x}",
                            parsed_config.vendor_id, parsed_config.product_id
                        ),
                    )
                })?;

        handle
            .claim_interface(parsed_config.interface)
            .map_err(|error| {
                io::Error::new(
                    io::ErrorKind::Other,
                    format!(
                        "failed to claim USB interface {}: {}",
                        parsed_config.interface, error
                    ),
                )
            })?;

        let read_timeout = Duration::from_millis(parsed_config.read_timeout_ms);
        let transact_timeout = Duration::from_millis(parsed_config.transact_timeout_ms);
        let buffer = vec![0_u8; parsed_config.buffer_size];

        Ok(Self {
            id,
            config: parsed_config,
            handle,
            read_timeout,
            transact_timeout,
            buffer,
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
            let read_size = match self.handle.read_bulk(
                self.config.endpoint_in,
                &mut self.buffer,
                self.read_timeout,
            ) {
                Ok(size) => size,
                Err(rusb::Error::Timeout) => {
                    return Ok(None);
                }
                Err(error) => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("USB bulk read failed: {error}"),
                    ));
                }
            };

            if read_size == 0 {
                return Ok(None);
            }

            let data = Bytes::copy_from_slice(&self.buffer[..read_size]);

            Ok(Some(TransportChunk {
                source_addr: self.synthetic_source_addr(),
                source_id: self.source_id(),
                data,
            }))
        })
    }

    fn transact_chunk(
        &mut self,
        request: TransportRequest,
    ) -> TransportFuture<'_, Vec<TransportChunk>> {
        Box::pin(async move {
            let written = self
                .handle
                .write_bulk(
                    self.config.endpoint_out,
                    &request.data,
                    self.transact_timeout,
                )
                .map_err(|error| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        format!("USB bulk write failed: {error}"),
                    )
                })?;

            if written != request.data.len() {
                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    format!(
                        "USB bulk write incomplete: written={}, expected={}",
                        written,
                        request.data.len()
                    ),
                ));
            }

            let expected_response_count = match request.response_mode {
                TransportResponseMode::None => {
                    return Ok(Vec::new());
                }
                TransportResponseMode::One => Some(1),
                TransportResponseMode::Count(count) => Some(count),
                TransportResponseMode::UntilTimeout => None,
            };

            let mut responses = Vec::new();

            loop {
                if let Some(expected_count) = expected_response_count {
                    if responses.len() >= expected_count {
                        break;
                    }
                }

                let read_size = match self.handle.read_bulk(
                    self.config.endpoint_in,
                    &mut self.buffer,
                    self.transact_timeout,
                ) {
                    Ok(size) => size,
                    Err(rusb::Error::Timeout) => {
                        break;
                    }
                    Err(error) => {
                        return Err(io::Error::new(
                            io::ErrorKind::Other,
                            format!("USB bulk read response failed: {error}"),
                        ));
                    }
                };

                if read_size == 0 {
                    break;
                }

                responses.push(TransportChunk {
                    source_addr: self.synthetic_source_addr(),
                    source_id: self.source_id(),
                    data: Bytes::copy_from_slice(&self.buffer[..read_size]),
                });
            }

            Ok(responses)
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
