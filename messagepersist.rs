#[derive(Debug, Queryable)]
pub struct Message {
    pub id: i32,
    pub sender_id: i32,
    pub recipient_id: i32,
    pub content: String,
    pub timestamp: NaiveDateTime,
}
