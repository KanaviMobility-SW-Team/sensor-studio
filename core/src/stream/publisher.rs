//! 스트림 데이터 발행 트레이트 모듈

use crate::types::PointCloudFrame;

/// 스트림 데이터 발행 기능 정의 인터페이스
pub trait StreamPublisher {
    fn publish(&mut self, frame: PointCloudFrame);
}
