use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::Response;
use axum::routing::get;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::{Mutex, broadcast};

use crate::stream::channel::ChannelRegistry;
use crate::stream::websocket::WebSocketMessage;
use crate::stream::websocket::foxglove::{
    FOXGLOVE_SUBPROTOCOL, FoxgloveClientCommand, FoxgloveClientMessage, encode_point_cloud_payload,
    foxglove_advertise_message, foxglove_server_info_message, make_message_data_frame,
};

#[derive(Clone)]
pub struct WebSocketServerState {
    pub sender: broadcast::Sender<WebSocketMessage>,
    pub channel_registry: Arc<ChannelRegistry>,
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
                )
            })
    }

    async fn handle_socket(
        socket: WebSocket,
        receiver: broadcast::Receiver<WebSocketMessage>,
        channel_registry: Arc<ChannelRegistry>,
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

        let subscriptions = Arc::new(Mutex::new(HashMap::<u32, u32>::new()));

        let send_subscriptions = Arc::clone(&subscriptions);
        let recv_subscriptions = Arc::clone(&subscriptions);

        let mut rx = receiver;
        let send_task = tokio::spawn(async move {
            while let Ok(message) = rx.recv().await {
                match message {
                    WebSocketMessage::Text(text) => {
                        if sender.send(Message::Text(text.into())).await.is_err() {
                            break;
                        }
                    }
                    WebSocketMessage::Frame { source_id, frame } => {
                        let Some(channel) = channel_registry.get_by_source(source_id) else {
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

                            if sender.send(Message::Binary(binary.into())).await.is_err() {
                                return;
                            }
                        }
                    }
                }
            }
        });

        let recv_task = tokio::spawn(async move {
            while let Some(Ok(message)) = incoming.next().await {
                match message {
                    Message::Text(text) => {
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
                                        subscriptions.insert(subscription_id, channel_id);
                                        println!(
                                            "subscribe: subscription_id={}, channel_id={}",
                                            subscription_id, channel_id
                                        );
                                    }
                                    FoxgloveClientCommand::Unsubscribe { subscription_id } => {
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
                    Message::Close(_) => break,
                    _ => {}
                }
            }
        });

        let _ = tokio::join!(send_task, recv_task);
    }
}
