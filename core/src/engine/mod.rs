use crate::types::PointCloudFrame;
use bytes::Bytes;

pub type EngineId = String;

pub trait Engine {
    fn id(&self) -> &str;

    fn process(&mut self, chunk: Bytes) -> Vec<PointCloudFrame>;
}
