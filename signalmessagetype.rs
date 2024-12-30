#[derive(Serialize, Deserialize)]
pub struct SignalingMessage {
    pub type_: String,       // "offer", "answer", or "candidate"
    pub sdp: Option<String>, // Session Description Protocol
    pub candidate: Option<String>,
    pub recipient_id: i32,
}
