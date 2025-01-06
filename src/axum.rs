use axum::{routing::post, Router};

async fn send_message(Json(payload): Json<SendMessageRequest>) -> impl IntoResponse {
    // Save message to DB
    let message = save_message_to_db(&payload).await?;
    Json(message)
}

fn app() -> Router {
    Router::new()
        .route("/messages", post(send_message))
}
