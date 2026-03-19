#[derive(Clone, Debug)]
pub struct InstanceRuntimeConfig {
    pub instance_id: String,
    pub channel: InstanceChannelConfig,
}

#[derive(Clone, Debug)]
pub struct InstanceChannelConfig {
    pub channel_id: u32,
    pub source_id: String,
    pub topic: String,
    pub schema: ChannelSchemaConfig,
    pub encoder: ChannelEncoderConfig,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChannelSchemaConfig {
    PointCloud,
    Status,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChannelEncoderConfig {
    Json,
}
