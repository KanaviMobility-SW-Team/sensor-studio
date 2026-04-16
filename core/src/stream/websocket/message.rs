//! 웹소켓 송수신 메시지 구조체 모듈

use crate::types::PointCloudFrame;

/// 브로드캐스트용 단일 포인트 클라우드 프레임 래퍼 전달 객체
#[derive(Clone, Debug)]
pub struct WebSocketMessage {
    pub source_id: String,
    pub frame: PointCloudFrame,
}
