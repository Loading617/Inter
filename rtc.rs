use dioxus::prelude::*;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use webrtc::api::APIBuilder;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::peer_connection::RTCPeerConnection;

#[tokio::main]
async fn main() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    let messages = use_state(cx, Vec::new);
    let input = use_state(cx, || "".to_string());
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    cx.spawn(async move {
        let (ws_stream, _) = connect_async("ws://localhost:3030/ws").await.unwrap();
        let (mut ws_tx, mut ws_rx) = ws_stream.split();

        while let Some(msg) = ws_rx.next().await {
            if let Ok(Message::Text(text)) = msg {
                tx.send(text).unwrap();
            }
        }
    });

    cx.spawn(async move {
        while let Some(msg) = rx.recv().await {
            messages.make_mut().push(msg);
        }
    });

    cx.render(rsx! {
        div {
            h1 { "Rust Messaging App" }
            div {
                for message in messages.get().iter() {
                    p { "{message}" }
                }
            }
            input {
                value: "{input}",
                oninput: move |evt| input.set(evt.value.clone()),
            }
            button {
                onclick: move |_| {
                    let text = input.get().clone();
                },
                "Send"
            }
            button {
                onclick: move |_| {
                    start_video_call();
                },
                "Start Video Call"
            }
        }
    })
}

async fn start_video_call() {
    let rtc_config = RTCConfiguration::default();
    let api = APIBuilder::new().build();
    let peer_connection = api.new_peer_connection(rtc_config).await.unwrap();

    let offer = peer_connection.create_offer(None).await.unwrap();
    peer_connection.set_local_description(offer).await.unwrap();

    println!("Send this SDP offer to the other peer: {}", peer_connection.local_description().await.unwrap().sdp);
}
