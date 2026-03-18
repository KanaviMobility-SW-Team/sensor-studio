use serde::Deserialize;
use serde_json::json;

pub const FOXGLOVE_SUBPROTOCOL: &str = "foxglove.websocket.v1";

pub fn foxglove_server_info_message() -> String {
    json!({
        "op": "serverInfo",
        "name": "sensor-studio-core",
        "capabilities": [],
        "supportedEncodings": [],
        "metadata": {}
    })
    .to_string()
}

pub fn foxglove_advertise_message() -> String {
    json!({
        "op": "advertise",
        "channels": [
            {
                "id": 1,
                "topic": "/pointcloud/mock",
                "encoding": "application/octet-stream",
                "schemaName": "sensor_msgs/msg/PointCloud2",
                "schema": ""
            }
        ]
    })
    .to_string()
}

#[derive(Debug, Deserialize)]
#[serde(tag = "op")]
pub enum FoxgloveClientMessage {
    #[serde(rename = "subscribe")]
    Subscribe {
        subscription_id: u32,
        channel_id: u32,
    },

    #[serde(rename = "unsubscribe")]
    Unsubscribe { subscription_id: u32 },
}
