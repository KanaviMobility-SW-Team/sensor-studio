use serde_json::json;

use crate::types::{PointCloudFrame, PointFieldDataType};

pub fn make_message_data_frame(subscription_id: u32, timestamp_ns: u64, payload: &[u8]) -> Vec<u8> {
    let mut frame = Vec::with_capacity(1 + 4 + 8 + payload.len());

    frame.push(0x01);
    frame.extend_from_slice(&subscription_id.to_le_bytes());
    frame.extend_from_slice(&timestamp_ns.to_le_bytes());
    frame.extend_from_slice(payload);

    frame
}

pub fn encode_point_cloud_payload(frame: &PointCloudFrame) -> Vec<u8> {
    let payload = json!({
        "timestamp": {
            "sec": frame.timestamp_ns / 1_000_000_000,
            "nsec": frame.timestamp_ns % 1_000_000_000,
        },
        "frame_id": frame.frame_id,
        "pose": {
            "position": { "x": 0.0, "y": 0.0, "z": 0.0 },
            "orientation": { "x": 0.0, "y": 0.0, "z": 0.0, "w": 1.0 }
        },
        "point_stride": frame.point_step,
        "fields": frame.fields.iter().map(|field| {
            json!({
                "name": field.name,
                "offset": field.offset,
                "type": to_foxglove_numeric_type(field.datatype),
            })
        }).collect::<Vec<_>>(),
        "data": frame.data,
    });

    serde_json::to_vec(&payload).unwrap_or_default()
}

fn to_foxglove_numeric_type(datatype: PointFieldDataType) -> u8 {
    match datatype {
        PointFieldDataType::Uint8 => 1,
        PointFieldDataType::Int8 => 2,
        PointFieldDataType::Uint16 => 3,
        PointFieldDataType::Int16 => 4,
        PointFieldDataType::Uint32 => 5,
        PointFieldDataType::Int32 => 6,
        PointFieldDataType::Float32 => 7,
        PointFieldDataType::Float64 => 8,
    }
}
