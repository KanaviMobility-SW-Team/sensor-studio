use crate::engine::EngineId;
use std::collections::HashMap;

pub type InstanceId = String;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InstanceState {
    Created,
    Running,
    Stopped,
    Error,
}

#[derive(Clone, Debug)]
pub struct Instance {
    pub id: InstanceId,
    pub engine_id: EngineId,
    pub state: InstanceState,
}

impl Instance {
    pub fn new(id: impl Into<String>, engine_id: EngineId) -> Self {
        Self {
            id: id.into(),
            engine_id,
            state: InstanceState::Created,
        }
    }
}

#[derive(Default)]
pub struct InstanceManager {
    instances: HashMap<InstanceId, Instance>,
}

impl InstanceManager {
    pub fn new() -> Self {
        Self {
            instances: HashMap::new(),
        }
    }

    pub fn add(&mut self, instance: Instance) -> Option<Instance> {
        self.instances.insert(instance.id.clone(), instance)
    }

    pub fn get(&self, id: &str) -> Option<&Instance> {
        self.instances.get(id)
    }

    pub fn get_mut(&mut self, id: &str) -> Option<&mut Instance> {
        self.instances.get_mut(id)
    }

    pub fn remove(&mut self, id: &str) -> Option<Instance> {
        self.instances.remove(id)
    }

    pub fn list(&self) -> Vec<&Instance> {
        self.instances.values().collect()
    }

    pub fn count(&self) -> usize {
        self.instances.len()
    }
}
