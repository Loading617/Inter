#[derive(Serialize, Deserialize, Clone)]
struct Message {
    sender: String,
    content: String,
    timestamp: String,
    device_info: String,
}

fn send_message(sender: String, content: String, device_info: String) -> Message {
    Message {
        sender,
        content,
        timestamp: chrono::Utc::now().to_string(),
        device_info,
    }
}

fn ChatMessage(cx: Scope<Message>) -> Element {
    cx.render(rsx!(
        div {
            class: "message",
            p { "{cx.props.sender}: {cx.props.content}" }
            small { "Sent from: {cx.props.device_info}" }
        }
    ))
}

#[post("/send_message")]
async fn send_message(req: web::Json<Message>) -> impl Responder {
    
    save_to_db(req.0).await;
    HttpResponse::Ok().json("Message Sent")
}
