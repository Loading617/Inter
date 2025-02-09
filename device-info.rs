use dioxus::prelude::*;

#[derive(Props, PartialEq)]
struct MessageProps {
    sender: String,
    content: String,
    device_info: String,
}

fn MessageBubble(cx: Scope<MessageProps>) -> Element {
    cx.render(rsx! {
        div { class: "message",
            p { "{cx.props.sender}: {cx.props.content}" }
            small { class: "device-info", "{cx.props.device_info}" }
        }
    })
}
