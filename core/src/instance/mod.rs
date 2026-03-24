use std::collections::HashMap;
use std::io;

use crate::engine::Engine;
use crate::transport::udp::UdpTransport;
use crate::types::PointCloudFrame;

pub type InstanceId = String;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InstanceState {
    Created,
    Running,
    Stopped,
    Error,
}

pub struct Instance {
    pub id: InstanceId,
    pub state: InstanceState,
    engine: Box<dyn Engine + Send>,
    transport: UdpTransport,
}

impl Instance {
    pub fn new(id: impl Into<String>, engine: Box<dyn Engine>, transport: UdpTransport) -> Self {
        Self {
            id: id.into(),
            state: InstanceState::Created,
            engine,
            transport,
        }
    }

    pub fn engine_id(&self) -> &str {
        self.engine.id()
    }

    pub fn transport_id(&self) -> &str {
        self.transport.id()
    }

    pub async fn run_once(&mut self) -> io::Result<Vec<PointCloudFrame>> {
        match self.transport.read_chunk().await? {
            Some((_sender_addr, chunk)) => {
                let frames = self.engine.process(chunk);
                Ok(frames)
            }
            None => Ok(Vec::new()),
        }
    }

    pub fn set_state(&mut self, state: InstanceState) {
        self.state = state;
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

    pub fn ids(&self) -> Vec<&str> {
        self.instances.keys().map(|k| k.as_str()).collect()
    }

    pub fn count(&self) -> usize {
        self.instances.len()
    }
}
