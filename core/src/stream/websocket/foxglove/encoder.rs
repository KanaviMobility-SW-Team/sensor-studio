use serde_json::json;

use crate::types::PointCloudFrame;

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
                "type": foxglove_field_type(field.datatype as u8),
            })
        }).collect::<Vec<_>>(),
        "data": frame.data,
    });

    serde_json::to_vec(&payload).unwrap_or_default()
}

fn foxglove_field_type(datatype: u8) -> &'static str {
    match datatype {
        1 => "int8",
        2 => "uint8",
        3 => "int16",
        4 => "uint16",
        5 => "int32",
        6 => "uint32",
        7 => "float32",
        8 => "float64",
        _ => "uint8",
    }
}
