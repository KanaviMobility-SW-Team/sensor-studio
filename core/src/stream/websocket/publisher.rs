use tokio::sync::broadcast;

use crate::stream::StreamPublisher;
use crate::stream::websocket::WebSocketMessage;
use crate::types::PointCloudFrame;

pub struct WebSocketPublisher {
    sender: broadcast::Sender<WebSocketMessage>,
    source_id: &'static str,
}

impl WebSocketPublisher {
    pub fn new(sender: broadcast::Sender<WebSocketMessage>, source_id: &'static str) -> Self {
        Self { sender, source_id }
    }
}

impl StreamPublisher for WebSocketPublisher {
    fn publish(&mut self, frame: PointCloudFrame) {
        let _ = self.sender.send(WebSocketMessage::Frame {
            source_id: self.source_id,
            frame,
        });
    }
}
