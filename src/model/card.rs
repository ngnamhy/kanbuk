#[derive(Debug, Clone)]
pub struct Card {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub deck_id: i32,
    pub position: i32,
    pub done: bool,
    pub created_at: String,
}
