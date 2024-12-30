use axum::extract::ws::{WebSocket, Message};
use tokio::sync::broadcast;

async fn signaling_handler(ws: WebSocket, state: SharedState) {
    let (mut sender, mut receiver) = ws.split();

    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Text(text) = msg {
            // Parse SDP or ICE candidate
            let signaling_data: SignalingMessage = serde_json::from_str(&text).unwrap();

            // Forward the message to the recipient via WebSocket
            if let Some(peer) = state.get_peer(&signaling_data.recipient_id) {
                peer.send(Message::Text(text.clone())).await.unwrap();
            }
        }
    }
}
