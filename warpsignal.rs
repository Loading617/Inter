use warp::Filter;
use std::{sync::Arc, collections::HashMap};
use tokio::sync::Mutex;
use warp::ws::{Message, WebSocket};
use futures::{StreamExt, SinkExt};
use webrtc::peer_connection::RTCPeerConnection;
use serde::{Serialize, Deserialize};

type Clients = Arc<Mutex<HashMap<String, warp::ws::Sender>>>>;

#[derive(Serialize, Deserialize)]
struct SignalMessage {
    sender: String,
    receiver: String,
    data: String,
}

#[tokio::main]
async fn main() {
    let clients = Clients::default();

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .map(|ws: warp::ws::Ws, clients| {
            ws.on_upgrade(move |socket| handle_connection(socket, clients))
        });

    warp::serve(ws_route).run(([127, 0, 0, 1], 3030)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

async fn handle_connection(ws: WebSocket, clients: Clients) {
    let (mut tx, mut rx) = ws.split();
    let id = uuid::Uuid::new_v4().to_string();
    
    clients.lock().await.insert(id.clone(), tx.clone());

    while let Some(result) = rx.next().await {
        if let Ok(msg) = result {
            if msg.is_text() {
                let text = msg.to_str().unwrap();
                let signal: SignalMessage = serde_json::from_str(text).unwrap();
                
                if let Some(client) = clients.lock().await.get(&signal.receiver) {
                    let _ = client.send(Message::text(text)).await;
                }
            }
        }
    }

    clients.lock().await.remove(&id);
}
