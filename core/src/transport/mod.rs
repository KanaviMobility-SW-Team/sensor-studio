//! 단말 센서 데이터 수신 및 전송 계층 모듈

pub mod udp;

use bytes::Bytes;

/// 트랜스포트 식별자 타입
pub type TransportId = String;

/// 통합 트랜스포트 분류 열거형
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransportKind {
    Udp,
    Usb,
    Serial,
}

/// 공통 트랜스포트 기능 정의 트레이트
pub trait Transport {
    fn id(&self) -> &str;

    fn kind(&self) -> TransportKind;

    fn read_chunk(&mut self) -> Option<Bytes>;
}
