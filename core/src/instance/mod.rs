//! 단일 센서 인스턴스 라이프사이클 관리 모듈
//!
//! 각각의 인스턴스는 한 대의 센서를 뜻함
//! 전송 계층(`transport`)을 통해 데이터를 수신하고, 엔진(`engine`)을 통해 이를
//! 포인트 클라우드로 변환하는 독립된 작업 단위

use std::collections::VecDeque;
use std::io;

use crate::engine::Engine;
use crate::transport::Transport;
use crate::types::PointCloudFrame;

/// 인스턴스 고유 식별자
pub type InstanceId = String;

const MAX_TRANSPORT_REQUESTS_PER_CYCLE: usize = 8;

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
    /// 수신된 청크 1개를 엔진에 전달하고, 엔진이 반환한 transport request를
    /// 제한된 횟수 내에서 처리한 뒤 생성된 PointCloud frame 목록을 반환한다.
    pub async fn run_once(&mut self) -> io::Result<Vec<PointCloudFrame>> {
        let Some(chunk) = self.transport.read_chunk().await? else {
            return Ok(Vec::new());
        };

        let mut frames = Vec::new();
        let mut pending_requests = VecDeque::new();

        let result = self.engine.process(chunk.data, chunk.source_addr);
        frames.extend(result.frames);
        pending_requests.extend(result.requests);

        let mut processed_requests = 0;

        while let Some(request) = pending_requests.pop_front() {
            if processed_requests >= MAX_TRANSPORT_REQUESTS_PER_CYCLE {
                tracing::warn!(
                    instance_id = %self.id,
                    max_requests = MAX_TRANSPORT_REQUESTS_PER_CYCLE,
                    "transport request processing limit reached"
                );
                break;
            }

            processed_requests += 1;

            let responses = self.transport.transact_chunk(request).await?;

            for response in responses {
                let response_result = self.engine.process(response.data, response.source_addr);

                frames.extend(response_result.frames);
                pending_requests.extend(response_result.requests);
            }
        }

        Ok(frames)
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
