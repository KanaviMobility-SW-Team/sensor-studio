//! 채널 디스크립터 구조체 및 정보 관리 모듈

use crate::config::{ChannelEncoderConfig, ChannelSchemaConfig, InstanceRuntimeConfig};

/// 단일 채널 설정 매니페스트
#[derive(Clone, Debug)]
pub struct ChannelDescriptor {
    pub id: u32,
    pub topic: String,
    pub source: ChannelSource,
    pub message_schema: ChannelSchema,
    pub encoder: ChannelEncoder,
}

/// 채널 데이터 소스 정보
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChannelSource {
    pub id: String,
}

/// 채널 데이터 스키마 타입
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChannelSchema {
    PointCloud,
    Status,
}

/// 채널 메세지 인코더 타입
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChannelEncoder {
    Json,
}

/// 전체 활성 채널 상태 레지스트리
#[derive(Clone, Debug)]
pub struct ChannelRegistry {
    channels: Vec<ChannelDescriptor>,
}

impl ChannelRegistry {
    pub fn new(channels: Vec<ChannelDescriptor>) -> Self {
        Self { channels }
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

    pub fn get_by_source(&self, source_id: &str) -> Option<&ChannelDescriptor> {
        self.channels
            .iter()
            .find(|channel| channel.source.id == source_id)
    }
}
