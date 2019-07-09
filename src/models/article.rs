use serde::*;

#[derive(Serialize)]
pub struct Article {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub tag_list: Vec<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub favorited: bool,
    pub favorites_count: u64,
    pub author_id: i64,
}
