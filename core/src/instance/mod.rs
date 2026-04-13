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
            Some((sender_addr, chunk)) => {
                let frames = self.engine.process(chunk, sender_addr);
                Ok(frames)
            }
            None => Ok(Vec::new()),
        }
    }

    pub fn set_state(&mut self, state: InstanceState) {
        self.state = state;
    }
}
