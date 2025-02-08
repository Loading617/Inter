use dioxus::prelude::*;
use gloo_net::websocket::{Message, WebSocket};

fn app(cx: Scope) -> Element {
    let messages = use_state(cx, Vec::new);
    let input_text = use_state(cx, || String::new());

    let ws = WebSocket::open("ws://localhost:3000/ws").unwrap();
    let messages_clone = messages.clone();

    cx.spawn({
        let ws = ws.clone();
        async move {
            while let Some(Ok(Message::Text(text))) = ws.next().await {
                messages_clone.modify(|msgs| msgs.push(text));
            }
        }
    });

    cx.render(rsx! {
        div {
            h1 { "Inter" }
            input {
                value: "{input_text}",
                oninput: |e| input_text.set(e.value.clone()),
            }
            button {
                onclick: move |_| {
                    ws.send(Message::Text(input_text.get().clone())).unwrap();
                    input_text.set("".to_string());
                },
                "Send"
            }
            ul {
                for msg in messages.iter() {
                    li { "{msg}" }
                }
            }
        }
    })
}

fn main() {
    dioxus_web::launch(app);
}
