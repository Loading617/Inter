use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    routing::get,
    response::IntoResponse,
    Router,
};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::broadcast;
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(100);
    let tx = Arc::new(tx);

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .layer(CorsLayer::new().allow_origin(Any));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(Message::Text(text)) = msg {
            println!("Received: {}", text);
            socket.send(Message::Text(format!("Echo: {}", text))).await.unwrap();
        }
    }
}
