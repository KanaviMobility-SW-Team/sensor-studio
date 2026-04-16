use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::runtime::adapter::FfiEngineAdapter;

/// 동시성 제어가 포함된 엔진 확장 제어(Extension) 공유 타입
pub type SharedEngineExtension = Arc<Mutex<FfiEngineAdapter>>;

/// 클라이언트의 제어 요청(JSON-RPC)을 특정 파서 엔진으로 라우팅하기 위한 전역 레지스트리
#[derive(Clone, Default)]
pub struct EngineExtensionRegistry {
    entries: Arc<HashMap<String, SharedEngineExtension>>,
}

impl EngineExtensionRegistry {
    /// 레지스트리 초기화 및 등록 매핑
    pub fn new(entries: HashMap<String, SharedEngineExtension>) -> Self {
        tracing::info!(
            "EngineExtensionRegistry initialized with {} extensions",
            entries.len()
        );
        Self {
            entries: Arc::new(entries),
        }
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn get(&self, instance_id: &str) -> Option<SharedEngineExtension> {
        self.entries.get(instance_id).cloned()
    }
}
