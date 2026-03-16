pub mod mock;

use bytes::Bytes;

use crate::types::PointCloudFrame;

pub type EngineId = String;

pub trait Engine {
    fn id(&self) -> &str;

    fn process(&mut self, chunk: Bytes) -> Vec<PointCloudFrame>;
}
