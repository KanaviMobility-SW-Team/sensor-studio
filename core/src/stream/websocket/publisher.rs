use tokio::sync::broadcast;

use crate::stream::StreamPublisher;
use crate::stream::websocket::WebSocketMessage;
use crate::types::PointCloudFrame;

pub struct WebSocketPublisher {
    sender: broadcast::Sender<WebSocketMessage>,
}

impl WebSocketPublisher {
    pub fn new(sender: broadcast::Sender<WebSocketMessage>) -> Self {
        Self { sender }
    }
}

impl StreamPublisher for WebSocketPublisher {
    fn publish(&mut self, frame: PointCloudFrame) {
        let message = WebSocketMessage::Text(format!(
            "frame: frame_id={}, points={}, data_size={}",
            frame.frame_id,
            frame.point_count(),
            frame.data.len()
        ));

        let _ = self.sender.send(message);
    }
}
