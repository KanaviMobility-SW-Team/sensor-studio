use serde::Deserialize;
use serde_json::{Value, json};

use crate::stream::channel::{ChannelDescriptor, ChannelEncoder, ChannelRegistry, ChannelSchema};

pub const FOXGLOVE_SUBPROTOCOL: &str = "foxglove.websocket.v1";

pub fn foxglove_server_info_message() -> String {
    json!({
        "op": "serverInfo",
        "name": "sensor-studio-core",
        "capabilities": [],
        "supportedEncodings": ["json"],
        "metadata": {}
    })
    .to_string()
}

fn foxglove_encoding(encoder: ChannelEncoder) -> &'static str {
    match encoder {
        ChannelEncoder::Json => "json",
    }
}

fn foxglove_schema_name(schema: ChannelSchema) -> &'static str {
    match schema {
        ChannelSchema::PointCloud => "foxglove.PointCloud",
        ChannelSchema::Status => "foxglove.RawMessage",
    }
}

fn channel_to_foxglove_advertise(channel: &ChannelDescriptor) -> Value {
    json!({
        "id": channel.id,
        "topic": channel.topic,
        "encoding": foxglove_encoding(channel.encoder),
        "schemaName": foxglove_schema_name(channel.message_schema),
        "schema": "",
    })
}

pub fn foxglove_advertise_message(registry: &ChannelRegistry) -> String {
    let channels = registry
        .channels()
        .iter()
        .map(channel_to_foxglove_advertise)
        .collect::<Vec<_>>();

    json!({
        "op": "advertise",
        "channels": channels,
    })
    .to_string()
}

#[derive(Debug)]
pub enum FoxgloveClientCommand {
    Subscribe {
        subscription_id: u32,
        channel_id: u32,
    },
    Unsubscribe {
        subscription_id: u32,
    },
}

#[derive(Debug, Deserialize)]
pub struct FoxgloveClientMessage {
    pub op: String,

    #[serde(default)]
    pub subscription_id: Option<u32>,

    #[serde(default)]
    pub channel_id: Option<u32>,

    #[serde(default)]
    pub subscriptions: Vec<FoxgloveSubscription>,

    #[serde(default, rename = "subscriptionIds")]
    pub subscription_ids: Vec<u32>,
}

#[derive(Debug, Deserialize)]
pub struct FoxgloveSubscription {
    pub id: u32,

    #[serde(rename = "channelId")]
    pub channel_id: u32,
}

impl FoxgloveClientMessage {
    pub fn into_commands(self) -> Vec<FoxgloveClientCommand> {
        match self.op.as_str() {
            "subscribe" => {
                if !self.subscriptions.is_empty() {
                    self.subscriptions
                        .into_iter()
                        .map(|subscription| FoxgloveClientCommand::Subscribe {
                            subscription_id: subscription.id,
                            channel_id: subscription.channel_id,
                        })
                        .collect()
                } else if let (Some(subscription_id), Some(channel_id)) =
                    (self.subscription_id, self.channel_id)
                {
                    vec![FoxgloveClientCommand::Subscribe {
                        subscription_id,
                        channel_id,
                    }]
                } else {
                    Vec::new()
                }
            }
            "unsubscribe" => {
                if !self.subscription_ids.is_empty() {
                    self.subscription_ids
                        .into_iter()
                        .map(|subscription_id| FoxgloveClientCommand::Unsubscribe {
                            subscription_id,
                        })
                        .collect()
                } else if let Some(subscription_id) = self.subscription_id {
                    vec![FoxgloveClientCommand::Unsubscribe { subscription_id }]
                } else {
                    Vec::new()
                }
            }
            _ => Vec::new(),
        }
    }
}
