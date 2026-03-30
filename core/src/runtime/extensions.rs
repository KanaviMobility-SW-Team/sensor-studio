use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::runtime::adapter::FfiEngineAdapter;

pub type SharedEngineExtension = Arc<Mutex<FfiEngineAdapter>>;

#[derive(Clone, Default)]
pub struct EngineExtensionRegistry {
    entries: Arc<HashMap<String, SharedEngineExtension>>,
}

impl EngineExtensionRegistry {
    pub fn new(entries: HashMap<String, SharedEngineExtension>) -> Self {
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
