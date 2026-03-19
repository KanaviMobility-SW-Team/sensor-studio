use crate::types::PointCloudFrame;

#[derive(Clone, Debug)]
pub enum WebSocketMessage {
    Text(String),
    Frame {
        source_id: &'static str,
        frame: PointCloudFrame,
    },
}
