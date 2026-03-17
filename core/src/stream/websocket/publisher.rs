use crate::stream::StreamPublisher;
use crate::types::PointCloudFrame;

pub struct WebSocketPublisher;

impl WebSocketPublisher {
    pub fn new() -> Self {
        Self
    }
}

impl StreamPublisher for WebSocketPublisher {
    fn publish(&mut self, _frame: PointCloudFrame) {
        // TODO: implement Websocket publishing logic here
    }
}
