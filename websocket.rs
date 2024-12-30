use tokio::sync::mpsc;
use axum::extract::ws::{WebSocket, Message};

pub async fn websocket_handler(ws: WebSocket) {
    let (mut sender, mut receiver) = ws.split();
    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            sender.send(Message::Text(format!("Echo: {}", text))).await.unwrap();
        }
    }
}
