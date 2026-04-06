use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::Response;
use axum::routing::get;
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::Value;
use tokio::sync::{Mutex, broadcast};

use crate::runtime::extensions::EngineExtensionRegistry;
use crate::stream::channel::ChannelRegistry;
use crate::stream::websocket::WebSocketMessage;
use crate::stream::websocket::foxglove::{
    FOXGLOVE_SUBPROTOCOL, FoxgloveClientCommand, FoxgloveClientMessage, encode_point_cloud_payload,
    foxglove_advertise_message, foxglove_server_info_message, make_message_data_frame,
};
use crate::stream::websocket::protocol::{
    ClientControlMessage, EngineExtensionApiInfoDto, ServerControlMessage,
};

#[derive(Debug, Deserialize)]
struct MessageOpEnvelope {
    op: String,
}

#[derive(Clone)]
pub struct WebSocketServerState {
    pub sender: broadcast::Sender<WebSocketMessage>,
    pub channel_registry: Arc<ChannelRegistry>,
    pub extension_registry: EngineExtensionRegistry,
}

pub struct WebSocketServer;

impl WebSocketServer {
    pub fn router(state: WebSocketServerState) -> Router {
        Router::new()
            .route("/ws", get(Self::ws_handler))
            .with_state(state)
    }

    pub async fn serve(
        bind_addr: SocketAddr,
        state: WebSocketServerState,
    ) -> Result<(), std::io::Error> {
        let listener = tokio::net::TcpListener::bind(bind_addr).await?;
        axum::serve(listener, Self::router(state))
            .await
            .map_err(std::io::Error::other)
    }

    async fn ws_handler(
        State(state): State<WebSocketServerState>,
        ws: WebSocketUpgrade,
    ) -> Response {
        ws.protocols([FOXGLOVE_SUBPROTOCOL])
            .on_upgrade(move |socket| {
                Self::handle_socket(
                    socket,
                    state.sender.subscribe(),
                    state.channel_registry.clone(),
                    state.extension_registry.clone(),
                )
            })
    }

    async fn handle_socket(
        socket: WebSocket,
        receiver: broadcast::Receiver<WebSocketMessage>,
        channel_registry: Arc<ChannelRegistry>,
        extension_registry: EngineExtensionRegistry,
    ) {
        let (mut sender, mut incoming) = socket.split();

        if sender
            .send(Message::Text(foxglove_server_info_message().into()))
            .await
            .is_err()
        {
            return;
        }

        if sender
            .send(Message::Text(
                foxglove_advertise_message(channel_registry.as_ref()).into(),
            ))
            .await
            .is_err()
        {
            return;
        }

        let (out_tx, mut out_rx) = tokio::sync::mpsc::channel::<Message>(64);

        tokio::spawn(async move {
            while let Some(message) = out_rx.recv().await {
                if sender.send(message).await.is_err() {
                    break;
                }
            }
        });

        let subscriptions = Arc::new(Mutex::new(HashMap::<u32, u32>::new()));

        let send_subscriptions = Arc::clone(&subscriptions);
        let recv_subscriptions = Arc::clone(&subscriptions);

        let mut rx = receiver;
        let out_tx_clone = out_tx.clone();
        let send_task = tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Ok(WebSocketMessage { source_id, frame }) => {
                        let Some(channel) = channel_registry.get_by_source(source_id.as_str())
                        else {
                            continue;
                        };

                        let subscriptions = send_subscriptions.lock().await;

                        for (subscription_id, channel_id) in subscriptions.iter() {
                            if *channel_id != channel.id {
                                continue;
                            }

                            let timestamp_ns = frame.timestamp_ns;
                            let payload = encode_point_cloud_payload(&frame);
                            let binary =
                                make_message_data_frame(*subscription_id, timestamp_ns, &payload);

                            if let Err(err) = out_tx_clone.try_send(Message::Binary(binary.into()))
                            {
                                if let tokio::sync::mpsc::error::TrySendError::Full(_) = err {
                                    eprintln!(
                                        "websocket outbound queue full for client. dropping frame for subscription_id={}",
                                        *subscription_id
                                    );
                                } else {
                                    // 채널이 닫혔을 때(Closed 오류 등)
                                    break;
                                }
                            }
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(count)) => {
                        eprintln!("websocket client lagged, skipped {count} messages");
                        continue;
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        break;
                    }
                }
            }
        });

        let out_tx_clone = out_tx.clone();
        let recv_task = tokio::spawn(async move {
            while let Some(Ok(message)) = incoming.next().await {
                match message {
                    Message::Text(text) => {
                        if let Ok(envelope) = serde_json::from_str::<MessageOpEnvelope>(&text) {
                            match envelope.op.as_str() {
                                "subscribe" | "unsubscribe" => {
                                    if let Ok(client_message) =
                                        serde_json::from_str::<FoxgloveClientMessage>(&text)
                                    {
                                        let commands = client_message.into_commands();
                                        let mut subscriptions = recv_subscriptions.lock().await;

                                        for command in commands {
                                            match command {
                                                FoxgloveClientCommand::Subscribe {
                                                    subscription_id,
                                                    channel_id,
                                                } => {
                                                    subscriptions
                                                        .insert(subscription_id, channel_id);
                                                    println!(
                                                        "subscribe: subscription_id={}, channel_id={}",
                                                        subscription_id, channel_id
                                                    );
                                                }
                                                FoxgloveClientCommand::Unsubscribe {
                                                    subscription_id,
                                                } => {
                                                    subscriptions.remove(&subscription_id);
                                                    println!(
                                                        "unsubscribe: subscription_id={}",
                                                        subscription_id
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    if let Some(response) =
                                        Self::handle_control_message(&text, &extension_registry)
                                            .await
                                    {
                                        if out_tx_clone
                                            .send(Message::Text(response.into()))
                                            .await
                                            .is_err()
                                        {
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Message::Close(_) => break,
                    _ => {}
                }
            }
        });

        let _ = tokio::join!(send_task, recv_task);
    }

    async fn handle_control_message(
        text: &str,
        extension_registry: &EngineExtensionRegistry,
    ) -> Option<String> {
        let request = match serde_json::from_str::<ClientControlMessage>(text) {
            Ok(value) => value,
            Err(error) => {
                let response = ServerControlMessage::EngineApiError {
                    instance_id: String::new(),
                    api_name: None,
                    message: format!("invalid control message: {error}"),
                };
                return serde_json::to_string(&response).ok();
            }
        };

        match request {
            ClientControlMessage::GetEngineApis { instance_id } => {
                let Some(shared) = extension_registry.get(&instance_id) else {
                    let response = ServerControlMessage::EngineApiError {
                        instance_id,
                        api_name: None,
                        message: "instance not found".to_string(),
                    };
                    return serde_json::to_string(&response).ok();
                };

                let result = tokio::task::block_in_place(|| match shared.lock() {
                    Ok(adapter) => adapter.list_extension_apis(),
                    Err(error) => Err(format!("failed to lock engine adapter: {error}").into()),
                });

                match result {
                    Ok(apis) => {
                        let response = ServerControlMessage::EngineApis {
                            instance_id,
                            apis: apis
                                .into_iter()
                                .map(EngineExtensionApiInfoDto::from)
                                .collect(),
                        };
                        serde_json::to_string(&response).ok()
                    }
                    Err(error) => {
                        let response = ServerControlMessage::EngineApiError {
                            instance_id,
                            api_name: None,
                            message: error.to_string(),
                        };
                        serde_json::to_string(&response).ok()
                    }
                }
            }

            ClientControlMessage::CallEngineApi {
                instance_id,
                api_name,
                input,
            } => {
                let Some(shared) = extension_registry.get(&instance_id) else {
                    let response = ServerControlMessage::EngineApiError {
                        instance_id,
                        api_name: Some(api_name),
                        message: "instance not found".to_string(),
                    };
                    return serde_json::to_string(&response).ok();
                };

                let input_json = match serde_json::to_string(&input) {
                    Ok(value) => value,
                    Err(error) => {
                        let response = ServerControlMessage::EngineApiError {
                            instance_id,
                            api_name: Some(api_name),
                            message: format!("failed to serialize input: {error}"),
                        };
                        return serde_json::to_string(&response).ok();
                    }
                };

                let result = tokio::task::block_in_place(|| match shared.lock() {
                    Ok(adapter) => adapter.call_extension_api(&api_name, &input_json),
                    Err(error) => Err(format!("failed to lock engine adapter: {error}").into()),
                });

                match result {
                    Ok(output_json) => {
                        let output = match serde_json::from_str::<Value>(&output_json) {
                            Ok(value) => value,
                            Err(_) => Value::String(output_json),
                        };

                        let response = ServerControlMessage::EngineApiResult {
                            instance_id,
                            api_name,
                            output,
                        };
                        serde_json::to_string(&response).ok()
                    }
                    Err(error) => {
                        let response = ServerControlMessage::EngineApiError {
                            instance_id,
                            api_name: Some(api_name),
                            message: error.to_string(),
                        };
                        serde_json::to_string(&response).ok()
                    }
                }
            }
        }
    }
}
