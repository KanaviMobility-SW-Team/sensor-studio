use std::net::SocketAddr;

use axum::Router;
use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::Response;
use axum::routing::get;
use tokio::sync::broadcast;

use crate::stream::websocket::WebSocketMessage;

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
        ws.on_upgrade(move |socket| Self::handle_socket(socket, state.sender.subscribe()))
    }

    async fn handle_socket(
        mut socket: WebSocket,
        mut receiver: broadcast::Receiver<WebSocketMessage>,
    ) {
        while let Ok(message) = receiver.recv().await {
            let send_result = match message {
                WebSocketMessage::Text(text) => socket.send(Message::Text(text.into())).await,
            };

            if send_result.is_err() {
                break;
            }
        }
    }
}
