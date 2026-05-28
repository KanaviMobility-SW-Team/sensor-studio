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
    Binary,
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
            .flat_map(|config| {
                let channel = &config.channel;

                let schema = match channel.schema {
                    ChannelSchemaConfig::PointCloud => ChannelSchema::PointCloud,
                    ChannelSchemaConfig::Status => ChannelSchema::Status,
                };
                let encoder = match channel.encoder {
                    ChannelEncoderConfig::Json => ChannelEncoder::Json,
                };
                let source = ChannelSource {
                    id: channel.source_id.clone(),
                };

                let json_channel = ChannelDescriptor {
                    id: channel.channel_id,
                    topic: channel.topic.clone(),
                    source: source.clone(),
                    message_schema: schema,
                    encoder,
                };

                let mut result = vec![json_channel];

                // PointCloud Json 채널에 대해 binary 쌍 채널 자동 생성
                if schema == ChannelSchema::PointCloud && encoder == ChannelEncoder::Json {
                    result.push(ChannelDescriptor {
                        id: channel.channel_id + 0x8000,
                        topic: format!("{}/raw", channel.topic),
                        source,
                        message_schema: schema,
                        encoder: ChannelEncoder::Binary,
                    });
                }

                result
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

    pub fn get_all_by_source(&self, source_id: &str) -> Vec<&ChannelDescriptor> {
        self.channels
            .iter()
            .filter(|channel| channel.source.id == source_id)
            .collect()
    }
}
