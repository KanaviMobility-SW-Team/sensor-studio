use std::net::SocketAddr;

use bytes::Bytes;

use crate::types::PointCloudFrame;

pub type EngineId = String;

pub trait Engine: Send {
    fn id(&self) -> &str;

    fn process(&mut self, chunk: Bytes, sender_addr: SocketAddr) -> Vec<PointCloudFrame>;
}
