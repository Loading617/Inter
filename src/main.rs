use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    let messages = use_state(cx, Vec::new);
    let input = use_state(cx, || "".to_string());

    cx.render(rsx! {
        div {
            h1 { "Inter" }
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
                    let mut new_messages = messages.get().clone();
                    new_messages.push(input.get().clone());
                    messages.set(new_messages);
                    input.set("".to_string());
                },
                "Send"
            }
        }
    })
}

