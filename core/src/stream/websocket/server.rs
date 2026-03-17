use std::net::SocketAddr;

use axum::Router;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::Response;
use axum::routing::get;

pub struct WebSocketServer;

impl WebSocketServer {
    pub fn router() -> Router {
        Router::new().route("/ws", get(Self::ws_handler))
    }

    pub async fn serve(bind_addr: SocketAddr) -> Result<(), std::io::Error> {
        let listener = tokio::net::TcpListener::bind(bind_addr).await?;
        axum::serve(listener, Self::router())
            .await
            .map_err(std::io::Error::other)
    }

    async fn ws_handler(ws: WebSocketUpgrade) -> Response {
        ws.on_upgrade(Self::handle_socket)
    }

    async fn handle_socket(mut socket: WebSocket) {
        let message = Message::Text("connected".to_string().into());
        if let Err(e) = socket.send(message).await {
            eprintln!("Error sending message: {:?}", e);
            return;
        }
    }
}
