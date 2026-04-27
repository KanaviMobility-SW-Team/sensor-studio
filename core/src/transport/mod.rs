//! 단말 센서 데이터 수신 및 전송 계층 모듈

pub mod udp;

use std::future::Future;
use std::io;
use std::net::SocketAddr;
use std::pin::Pin;

use bytes::Bytes;

/// 트랜스포트 식별자 타입
pub type TransportId = String;

/// 비동기 트랜스포트 작업 결과 타입
pub type TransportFuture<'a, T> = Pin<Box<dyn Future<Output = io::Result<T>> + Send + 'a>>;

/// 통합 트랜스포트 분류 열거형
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransportKind {
    Udp,
    Usb,
    Serial,
}

/// 트랜스포트에서 읽은 원시 데이터 청크
#[derive(Debug, Clone)]
pub struct TransportChunk {
    pub source_addr: SocketAddr,
    pub source_id: String,
    pub data: Bytes,
}

#[derive(Debug, Clone)]
pub enum TransportResponseMode {
    None,
    One,
    Count(usize),
    UntilTimeout,
}

impl Default for TransportResponseMode {
    fn default() -> Self {
        Self::One
    }
}

/// 트랜스포트로 전송할 원시 데이터 청크
#[derive(Debug, Clone)]
pub struct TransportRequest {
    pub target_addr: Option<SocketAddr>,
    pub target_id: Option<String>,
    pub data: Bytes,
    pub response_mode: TransportResponseMode,
}

pub trait Transport {
    fn id(&self) -> &str;

    fn kind(&self) -> TransportKind;

    fn read_chunk(&mut self) -> TransportFuture<'_, Option<TransportChunk>>;

    fn transact_chunk(
        &mut self,
        request: TransportRequest,
    ) -> TransportFuture<'_, Vec<TransportChunk>>;
}
