use crate::stream::StreamPublisher;
use crate::types::PointCloudFrame;

pub struct ConsolePublisher;

impl ConsolePublisher {
    pub fn new() -> Self {
        Self
    }
}

impl StreamPublisher for ConsolePublisher {
    fn publish(&mut self, frame: PointCloudFrame) {
        println!(
            "published frame: frame_id={}, points={}, data_size={} bytes",
            frame.frame_id,
            frame.point_count(),
            frame.data.len()
        );
    }
}
