use serde::*;

#[derive(Serialize, Queryable)]
pub struct Article {
    pub id: i64,
    pub author_id: i64,
    pub description: String,
    pub favorites_count: i32,
    pub slug: String,
    pub tag_list: Vec<String>,
    pub title: String,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
