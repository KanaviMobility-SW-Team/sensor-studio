#[derive(Clone, Debug)]
pub struct ChannelDescriptor {
    pub id: u32,
    pub topic: &'static str,
    pub message_kind: ChannelMessageKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChannelMessageKind {
    PointCloud,
    Status,
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
            message_kind: ChannelMessageKind::PointCloud,
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
}
