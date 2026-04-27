use std::net::SocketAddr;

use bytes::Bytes;

use crate::transport::TransportRequest;
use crate::types::PointCloudFrame;

/// 엔진 고유 식별자
pub type EngineId = String;

#[derive(Debug, Default)]
pub struct EngineProcessResult {
    pub frames: Vec<PointCloudFrame>,
    pub requests: Vec<TransportRequest>,
}

impl EngineProcessResult {
    pub fn from_frames(frames: Vec<PointCloudFrame>) -> Self {
        Self {
            frames,
            requests: Vec::new(),
        }
    }

    pub fn empty() -> Self {
        Self::default()
    }
}

/// 센서 데이터 파싱을 담당하는 C-FFI 플러그인 통신 인터페이스
pub trait Engine: Send {
    /// 엔진 ID 반환
    fn id(&self) -> &str;

    /// 수신된 바이트 청크와 송신지 IP(`sender_addr`)를 바탕으로 PointCloud 데이터 변환
    fn process(&mut self, chunk: Bytes, sender_addr: SocketAddr) -> EngineProcessResult;
}
