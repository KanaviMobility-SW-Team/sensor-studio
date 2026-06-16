//! 웹소켓 프로토콜 서버 모듈

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
use crate::stream::channel::ChannelEncoder;
use crate::stream::channel::ChannelRegistry;
use crate::stream::websocket::WebSocketMessage;
use crate::stream::websocket::foxglove::{
    FOXGLOVE_SUBPROTOCOL, FoxgloveClientCommand, FoxgloveClientMessage, encode_point_cloud_payload,
    encode_point_cloud_payload_binary, foxglove_advertise_message, foxglove_server_info_message,
    make_message_data_frame,
};
use crate::stream::websocket::protocol::{
    ClientControlMessage, EngineExtensionApiInfoDto, ServerControlMessage,
};

/// 메시지 제어 연산 분석 래퍼
#[derive(Debug, Deserialize)]
struct MessageOpEnvelope {
    op: String,
}

/// 웹소켓 서버 전역 상태
#[derive(Clone)]
pub struct WebSocketServerState {
    pub sender: broadcast::Sender<WebSocketMessage>,
    pub channel_registry: Arc<ChannelRegistry>,
    pub extension_registry: EngineExtensionRegistry,
}

/// 웹소켓 서버 핸들러
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

        // ====================================================================
        // [개선된 아키텍처]: Queue -> State(전광판) 방식으로 완벽 교체
        // ====================================================================
        // 1. 제어 메시지용 큐 (무제한 큐: API 응답 등은 절대 유실되면 안 됨)
        let (control_tx, mut control_rx) = tokio::sync::mpsc::unbounded_channel::<Message>();

        // 2. 센서 데이터 보관용 전광판 (센서(SubID)별로 최신 1프레임만 유지)
        let pending_frames = Arc::new(Mutex::new(HashMap::<u32, Message>::new()));

        // 3. 전송 알리미 (새 데이터가 덮어씌워졌음을 소비자에게 알림)
        let notify = Arc::new(tokio::sync::Notify::new());

        let pending_frames_clone = pending_frames.clone();
        let notify_clone = notify.clone();

        // 웹소켓 전송 전담
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    // 우선순위 1: 제어 메시지가 오면 즉시 전송
                    Some(ctrl_msg) = control_rx.recv() => {
                        if sender.send(ctrl_msg).await.is_err() { break; }
                    }
                    // 우선순위 2: 새 센서 데이터가 덮어씌워졌다는 알림을 받으면
                    _ = notify_clone.notified() => {
                        // 전광판(Map)에 걸려있는 '모든 센서의 가장 최신 프레임'을 싹 수거하고 맵을 비움
                        let frames_to_send: Vec<Message> = {
                            let mut map = pending_frames_clone.lock().await;
                            map.drain().map(|(_, msg)| msg).collect()
                        };

                        // 수거한 최신 프레임들을 지연 없이 전송
                        for msg in frames_to_send {
                            if sender.send(msg).await.is_err() { return; }
                        }
                    }
                }
            }
        });

        let subscriptions = Arc::new(Mutex::new(HashMap::<u32, u32>::new()));
        let send_subscriptions = Arc::clone(&subscriptions);
        let recv_subscriptions = Arc::clone(&subscriptions);

        let mut rx = receiver;
        let pending_frames_tx = pending_frames.clone();
        let notify_tx = notify.clone();

        // 데이터 수신 및 인코딩 전담
        let send_task = tokio::spawn(async move {
            loop {
                match rx.recv().await {
                    Ok(WebSocketMessage { source_id, frame }) => {
                        let channels = channel_registry.get_all_by_source(source_id.as_str());
                        if channels.is_empty() {
                            continue;
                        }

                        let subscriptions = send_subscriptions.lock().await;

                        for channel in &channels {
                            let mut payload_encoded = false;
                            let mut payload = Vec::new();

                            for (subscription_id, channel_id) in subscriptions.iter() {
                                if *channel_id != channel.id {
                                    continue;
                                }

                                // 지연 인코딩 최적화
                                if !payload_encoded {
                                    payload = match channel.encoder {
                                        ChannelEncoder::Json => encode_point_cloud_payload(&frame),
                                        ChannelEncoder::Binary => {
                                            encode_point_cloud_payload_binary(&frame)
                                        }
                                    };
                                    payload_encoded = true;
                                }

                                let binary = make_message_data_frame(
                                    *subscription_id,
                                    frame.timestamp_ns,
                                    &payload,
                                );

                                // 큐에 줄을 세우지 않고, 센서(SubID)별 자기 자리에 "최신 데이터 덮어쓰기"
                                pending_frames_tx
                                    .lock()
                                    .await
                                    .insert(*subscription_id, Message::Binary(binary.into()));

                                // 데이터 갱신 완료 알림 (웹소켓 전송작업이 대기 중이면 즉각 깨움)
                                notify_tx.notify_one();
                            }
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(count)) => {
                        tracing::warn!("websocket broadcast lagged, skipped {count} frames");
                        continue;
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        break;
                    }
                }
            }
        });

        let control_tx_clone = control_tx.clone();
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
                                                }
                                                FoxgloveClientCommand::Unsubscribe {
                                                    subscription_id,
                                                } => {
                                                    subscriptions.remove(&subscription_id);
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
                                        // 제어 메시지는 유실되지 않도록 control_tx로 보냅니다.
                                        let _ =
                                            control_tx_clone.send(Message::Text(response.into()));
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
