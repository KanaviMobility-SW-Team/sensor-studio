//! 단일 센서 인스턴스 라이프사이클 관리 모듈
//!
//! 각각의 인스턴스는 한 대의 센서를 뜻함
//! 전송 계층(`transport`)을 통해 데이터를 수신하고, 엔진(`engine`)을 통해 이를
//! 포인트 클라우드로 변환하는 독립된 작업 단위

use std::io;

use crate::engine::Engine;
use crate::transport::Transport;
use crate::types::PointCloudFrame;

/// 인스턴스 고유 식별자
pub type InstanceId = String;

/// 센서 인스턴스 동작 상태
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InstanceState {
    /// 런타임 설정에 의해 생성된 초기 상태
    Created,
    /// 패킷 수신 및 정상 처리 중
    Running,
    /// 사용자 요청으로 동작 중지
    Stopped,
    /// 런타임 오류로 수집 및 변환 실패 (백오프 재시도 모드)
    Error,
}

/// 개별 센서 하드웨어/스트림과 1:1 매핑되는 핵심 구조체
pub struct Instance {
    /// 인스턴스 고유 식별자 (`runtime.toml` 내 지정값)
    pub id: InstanceId,
    /// 현재 상태
    pub state: InstanceState,

    /// 데이터를 PointCloud로 변환하는 어댑터 엔진 인터페이스
    engine: Box<dyn Engine + Send>,
    /// 센서 패킷 수신 네트워크 레이어
    transport: Box<dyn Transport + Send>,
}

impl Instance {
    /// 인스턴스 초기화
    pub fn new(
        id: impl Into<String>,
        engine: Box<dyn Engine + Send>,
        transport: Box<dyn Transport + Send>,
    ) -> Self {
        Self {
            id: id.into(),
            state: InstanceState::Created,
            engine,
            transport,
        }
    }

    /// 바인딩된 엔진 ID 반환
    pub fn engine_id(&self) -> &str {
        self.engine.id()
    }

    /// 바인딩된 전송 계층 식별자 반환
    pub fn transport_id(&self) -> &str {
        self.transport.id()
    }

    /// 1회 사이클 실행
    ///
    /// 엔진이 보유한 transport request가 있으면 현재 transport로 먼저 전송하고,
    /// 응답 청크를 다시 엔진에 전달한다.
    /// 처리할 request가 없으면 transport에서 일반 수신 청크를 읽어 엔진에 전달한다.
    pub async fn run_once(&mut self) -> io::Result<Vec<PointCloudFrame>> {
        if let Some(request) = self.engine.pop_transport_request()? {
            tracing::info!(
                "Instance '{}' processing transport request payload size {} bytes",
                self.id,
                request.data.len()
            );

            let responses = self.transport.transact_chunk(request).await?;
            tracing::info!(
                "Instance '{}' received {} response(s) for transport request",
                self.id,
                responses.len()
            );

            let mut frames = Vec::new();

            for response in responses {
                let result = self.engine.process(response.data, response.source_addr);
                frames.extend(result.frames);
            }

            return Ok(frames);
        }

        let Some(chunk) = self.transport.read_chunk().await? else {
            return Ok(Vec::new());
        };

        let result = self.engine.process(chunk.data, chunk.source_addr);
        Ok(result.frames)
    }

    /// 인스턴스 상태 변경 및 로그 출력
    pub fn set_state(&mut self, state: InstanceState) {
        if self.state != state {
            if state == InstanceState::Error {
                tracing::warn!("Instance '{}' state changed to {:?}", self.id, state);
            } else {
                tracing::info!("Instance '{}' state changed to {:?}", self.id, state);
            }
            self.state = state;
        }
    }
}
