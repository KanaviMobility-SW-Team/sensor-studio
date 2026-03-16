use bytes::Bytes;

use crate::engine::Engine;
use crate::types::{PointCloudFrame, PointField, PointFieldDataType};

const POINT_STEP: u32 = 17;

pub struct MockEngine {
    id: String,
    frame_counter: u64,
}

impl MockEngine {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            frame_counter: 0,
        }
    }

    fn default_fields() -> Vec<PointField> {
        vec![
            PointField {
                name: "x".to_string(),
                offset: 0,
                datatype: PointFieldDataType::Float32,
                count: 1,
            },
            PointField {
                name: "y".to_string(),
                offset: 4,
                datatype: PointFieldDataType::Float32,
                count: 1,
            },
            PointField {
                name: "z".to_string(),
                offset: 8,
                datatype: PointFieldDataType::Float32,
                count: 1,
            },
            PointField {
                name: "intensity".to_string(),
                offset: 12,
                datatype: PointFieldDataType::Uint32,
                count: 1,
            },
            PointField {
                name: "echo".to_string(),
                offset: 16,
                datatype: PointFieldDataType::Uint8,
                count: 1,
            },
        ]
    }

    fn build_frame(&mut self, chunk_len: usize) -> PointCloudFrame {
        self.frame_counter += 1;

        let timestamp_ns = self.frame_counter;

        let x: f32 = 1.0;
        let y: f32 = 2.0;
        let z: f32 = 3.0;
        let intensity: u32 = chunk_len as u32;
        let echo: u8 = 0;

        let mut data = Vec::with_capacity(POINT_STEP as usize);
        data.extend_from_slice(&x.to_le_bytes());
        data.extend_from_slice(&y.to_le_bytes());
        data.extend_from_slice(&z.to_le_bytes());
        data.extend_from_slice(&intensity.to_le_bytes());
        data.push(echo);

        PointCloudFrame::new(
            timestamp_ns,
            "mock_sensor",
            1,
            1,
            POINT_STEP,
            Self::default_fields(),
            true,
            data,
        )
    }
}

impl Engine for MockEngine {
    fn id(&self) -> &str {
        &self.id
    }

    fn process(&mut self, chunk: Bytes) -> Vec<PointCloudFrame> {
        vec![self.build_frame(chunk.len())]
    }
}
