use crate::types::PointCloudFrame;

pub trait StreamPublisher {
    fn publish(&mut self, frame: PointCloudFrame);
}
