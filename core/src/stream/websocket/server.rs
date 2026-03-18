use std::collections::HashMap;
use std::net::SocketAddr;

use axum::Router;
use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::Response;
use axum::routing::get;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast;

use crate::stream::websocket::WebSocketMessage;
use crate::stream::websocket::foxglove::{
    FOXGLOVE_SUBPROTOCOL, FoxgloveClientMessage, foxglove_advertise_message,
    foxglove_server_info_message,
};

#[derive(Clone)]
pub struct WebSocketServerState {
    pub sender: broadcast::Sender<WebSocketMessage>,
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
            .on_upgrade(move |socket| Self::handle_socket(socket, state.sender.subscribe()))
    }

    async fn handle_socket(socket: WebSocket, receiver: broadcast::Receiver<WebSocketMessage>) {
        let (mut sender, mut incoming) = socket.split();

        if sender
            .send(Message::Text(foxglove_server_info_message().into()))
            .await
            .is_err()
        {
            return;
        }

        if sender
            .send(Message::Text(foxglove_advertise_message().into()))
            .await
            .is_err()
        {
            return;
        }

        let mut subscriptions: HashMap<u32, u32> = HashMap::new();

        let mut rx = receiver;
        let send_task = tokio::spawn(async move {
            while let Ok(message) = rx.recv().await {
                let text = match message {
                    WebSocketMessage::Text(text) => text,
                    WebSocketMessage::Frame(frame) => {
                        let point_count = frame.point_count();
                        let data_size = frame.data.len();
                        let frame_id = frame.frame_id;

                        format!(
                            "frame_id={frame_id}, point_count={point_count}, data_size={data_size}"
                        )
                    }
                };

                if sender.send(Message::Text(text.into())).await.is_err() {
                    break;
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
                            match client_message {
                                FoxgloveClientMessage::Subscribe {
                                    subscription_id,
                                    channel_id,
                                } => {
                                    subscriptions.insert(subscription_id, channel_id);
                                    println!(
                                        "Subscribed: subscription_id={}, channel_id={}",
                                        subscription_id, channel_id
                                    );
                                }
                                FoxgloveClientMessage::Unsubscribe { subscription_id } => {
                                    subscriptions.remove(&subscription_id);
                                    println!("Unsubscribed: subscription_id={}", subscription_id);
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
