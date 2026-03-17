use crate::types::PointCloudFrame;

#[derive(Clone, Debug)]
pub enum WebSocketMessage {
    Text(String),
    Frame(PointCloudFrame),
}
