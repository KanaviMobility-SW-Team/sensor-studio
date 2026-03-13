#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PointField {
    pub name: String,
    pub offset: u32,
    pub datatype: PointFieldDataType,
    pub count: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PointFieldDataType {
    Int8 = 1,
    Uint8 = 2,
    Int16 = 3,
    Uint16 = 4,
    Int32 = 5,
    Uint32 = 6,
    Float32 = 7,
    Float64 = 8,
}

#[derive(Clone, Debug)]
pub struct PointCloudFrame {
    pub timestamp_ns: u64,
    pub frame_id: String,
    pub width: u32,
    pub height: u32,
    pub point_step: u32,
    pub row_step: u32,
    pub fields: Vec<PointField>,
    pub is_dense: bool,
    pub data: Vec<u8>,
}

impl PointCloudFrame {
    pub fn new(
        timestamp_ns: u64,
        frame_id: impl Into<String>,
        width: u32,
        height: u32,
        point_step: u32,
        fields: Vec<PointField>,
        is_dense: bool,
        data: Vec<u8>,
    ) -> Self {
        let row_step = width.saturating_mul(point_step);

        Self {
            timestamp_ns,
            frame_id: frame_id.into(),
            width,
            height,
            point_step,
            row_step,
            fields,
            is_dense,
            data,
        }
    }

    pub fn point_count(&self) -> u32 {
        self.width.saturating_mul(self.height)
    }
}
