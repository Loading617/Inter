#[tokio::test]
async fn test_send_message() {
    let response = send_message(MockRequest::new(payload)).await;
    assert_eq!(response.status(), StatusCode::OK);
}
