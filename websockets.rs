use dioxus::prelude::*;
use tokio_tungstenite::connect_async;
use futures::stream::{SplitSink, SplitStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio::sync::mpsc;
use futures::SinkExt;

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
        }
    })
}
