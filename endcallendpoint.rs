#[derive(Deserialize)]
struct EndCallRequest {
    call_id: i32,
}

async fn end_call(Json(payload): Json<EndCallRequest>) -> impl IntoResponse {
    sqlx::query!(
        "UPDATE calls SET end_time = NOW(), status = 'completed' WHERE id = $1",
        payload.call_id
    )
    .execute(&db_pool)
    .await?;
    StatusCode::OK
}
