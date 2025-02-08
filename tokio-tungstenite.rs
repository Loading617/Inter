use tokio_tungstenite::connect_async;
use url::Url;

async fn connect_to_server() {
    let (ws_stream, _) = connect_async(Url::parse("ws://localhost:8080").unwrap()).await.unwrap();
    println!("Connected to the server!");
}
