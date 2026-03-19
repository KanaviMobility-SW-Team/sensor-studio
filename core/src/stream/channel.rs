#[derive(Clone, Debug)]
pub struct ChannelDescriptor {
    pub id: u32,
    pub topic: &'static str,
    pub source: ChannelSource,
    pub message_schema: ChannelSchema,
    pub encoder: ChannelEncoder,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChannelSource {
    pub id: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChannelSchema {
    PointCloud,
    Status,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChannelEncoder {
    Json,
}

#[derive(Clone, Debug)]
pub struct ChannelRegistry {
    channels: Vec<ChannelDescriptor>,
}

impl ChannelRegistry {
    pub fn new(channels: Vec<ChannelDescriptor>) -> Self {
        Self { channels }
    }

    pub fn mock_pointcloud() -> Self {
        Self::new(vec![ChannelDescriptor {
            id: 1,
            topic: "/pointcloud/mock",
            source: ChannelSource { id: "mock_sensor" },
            message_schema: ChannelSchema::PointCloud,
            encoder: ChannelEncoder::Json,
        }])
    }

    pub fn channels(&self) -> &[ChannelDescriptor] {
        &self.channels
    }

    pub fn contains(&self, channel_id: u32) -> bool {
        self.channels.iter().any(|channel| channel.id == channel_id)
    }

    pub fn get(&self, channel_id: u32) -> Option<&ChannelDescriptor> {
        self.channels
            .iter()
            .find(|channel| channel.id == channel_id)
    }

    pub fn get_by_source(&self, source_id: &str) -> Option<&ChannelDescriptor> {
        self.channels
            .iter()
            .find(|channel| channel.source.id == source_id)
    }
}
