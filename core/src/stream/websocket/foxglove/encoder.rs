//! Foxglove 프로토콜 메시지 데이터 직렬화 및 인코딩 모듈

use serde_json::json;

use crate::types::{PointCloudFrame, PointFieldDataType};

/// 바이너리 데이터 프레임 바이트 배열 생성
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
        PointFieldDataType::Int32 => 5,
        PointFieldDataType::Uint32 => 6,
        PointFieldDataType::Float32 => 7,
        PointFieldDataType::Float64 => 8,
        _default => 0, // Unknown or unsupported types
    }
}

/// 커스텀 바이너리 포맷으로 포인트 클라우드 직렬화
///
/// 포맷:
///   [point_stride: u32 LE]
///   [num_fields: u8]
///   for each field:
///     [name_len: u8][name: utf8][offset: u16 LE][type: u8]
///   [raw point bytes]
pub fn encode_point_cloud_payload_binary(frame: &PointCloudFrame) -> Vec<u8> {
    let mut buf = Vec::new();

    buf.extend_from_slice(&(frame.point_step as u32).to_le_bytes());
    buf.push(frame.fields.len() as u8);

    for field in &frame.fields {
        let name = field.name.as_bytes();
        buf.push(name.len() as u8);
        buf.extend_from_slice(name);
        buf.extend_from_slice(&(field.offset as u16).to_le_bytes());
        buf.push(to_foxglove_numeric_type(field.datatype));
    }

    buf.extend_from_slice(&frame.data);
    buf
}
