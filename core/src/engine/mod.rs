use std::net::SocketAddr;

use bytes::Bytes;

use crate::types::PointCloudFrame;

/// 엔진 고유 식별자
pub type EngineId = String;

/// 센서 데이터 파싱을 담당하는 C-FFI 플러그인 통신 인터페이스
pub trait Engine: Send {
    /// 엔진 ID 반환
    fn id(&self) -> &str;

    /// 수신된 바이트 청크와 송신지 IP(`sender_addr`)를 바탕으로 PointCloud 데이터 변환
    fn process(&mut self, chunk: Bytes, sender_addr: SocketAddr) -> Vec<PointCloudFrame>;
}
