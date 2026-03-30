use crate::types::PointCloudFrame;

#[derive(Clone, Debug)]
pub struct WebSocketMessage {
    pub source_id: String,
    pub frame: PointCloudFrame,
}
