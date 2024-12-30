use axum::Json;
use serde::Deserialize;

#[derive(Deserialize)]
struct StartCallRequest {
    caller_id: i32,
    receiver_id: i32,
}

async fn start_call(Json(payload): Json<StartCallRequest>) -> impl IntoResponse {
    let new_call = sqlx::query!(
        "INSERT INTO calls (caller_id, receiver_id, status) VALUES ($1, $2, 'initiated') RETURNING id",
        payload.caller_id,
        payload.receiver_id
    )
    .fetch_one(&db_pool)
    .await?;
    Json(new_call)
}
