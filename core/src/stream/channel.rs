use crate::config::{
    ChannelEncoderConfig, ChannelSchemaConfig, InstanceChannelConfig, InstanceRuntimeConfig,
};

#[derive(Clone, Debug)]
pub struct ChannelDescriptor {
    pub id: u32,
    pub topic: String,
    pub source: ChannelSource,
    pub message_schema: ChannelSchema,
    pub encoder: ChannelEncoder,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChannelSource {
    pub id: String,
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
            topic: "/pointcloud/mock".to_string(),
            source: ChannelSource {
                id: "mock_sensor".to_string(),
            },
            message_schema: ChannelSchema::PointCloud,
            encoder: ChannelEncoder::Json,
        }])
    }

    pub fn from_configs(configs: &[InstanceChannelConfig]) -> Self {
        let channels = configs
            .iter()
            .map(|config| ChannelDescriptor {
                id: config.channel_id,
                topic: config.topic.clone(),
                source: ChannelSource {
                    id: config.source_id.clone(),
                },
                message_schema: match config.schema {
                    ChannelSchemaConfig::PointCloud => ChannelSchema::PointCloud,
                    ChannelSchemaConfig::Status => ChannelSchema::Status,
                },
                encoder: match config.encoder {
                    ChannelEncoderConfig::Json => ChannelEncoder::Json,
                },
            })
            .collect();

        Self::new(channels)
    }

    pub fn from_instance_configs(configs: &[InstanceRuntimeConfig]) -> Self {
        let channels = configs
            .iter()
            .map(|config| {
                let channel = &config.channel;

                ChannelDescriptor {
                    id: channel.channel_id,
                    topic: channel.topic.clone(),
                    source: ChannelSource {
                        id: channel.source_id.clone(),
                    },
                    message_schema: match channel.schema {
                        ChannelSchemaConfig::PointCloud => ChannelSchema::PointCloud,
                        ChannelSchemaConfig::Status => ChannelSchema::Status,
                    },
                    encoder: match channel.encoder {
                        ChannelEncoderConfig::Json => ChannelEncoder::Json,
                    },
                }
            })
            .collect();

        Self::new(channels)
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

    pub fn get_by_source(&self, source_id: String) -> Option<&ChannelDescriptor> {
        self.channels
            .iter()
            .find(|channel| channel.source.id == source_id)
    }
}
