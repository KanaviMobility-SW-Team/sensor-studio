use std::net::SocketAddr;

use axum::Router;
use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::Response;
use axum::routing::get;
use tokio::sync::broadcast;

use crate::stream::websocket::WebSocketMessage;
use crate::stream::websocket::foxglove::{
    FOXGLOVE_SUBPROTOCOL, foxglove_advertise_message, foxglove_server_info_message,
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

    async fn handle_socket(
        mut socket: WebSocket,
        mut receiver: broadcast::Receiver<WebSocketMessage>,
    ) {
        if socket
            .send(Message::Text(foxglove_server_info_message().into()))
            .await
            .is_err()
        {
            return;
        }

        if socket
            .send(Message::Text(foxglove_advertise_message().into()))
            .await
            .is_err()
        {
            return;
        }

        while let Ok(message) = receiver.recv().await {
            let text = match message {
                WebSocketMessage::Text(text) => text,
                WebSocketMessage::Frame(frame) => {
                    let point_count = frame.point_count();
                    let data_size = frame.data.len();
                    let frame_id = frame.frame_id;

                    format!("frame_id={frame_id}, point_count={point_count}, data_size={data_size}")
                }
            };

            if socket.send(Message::Text(text.into())).await.is_err() {
                break;
            }
        }
    }
}
