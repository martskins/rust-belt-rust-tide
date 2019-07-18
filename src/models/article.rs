use serde::*;

use crate::schema::article;

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

#[derive(Deserialize, Insertable)]
#[table_name = "article"]
pub struct NewArticle {
    pub author_id: i64,
    pub description: String,
    pub slug: String,
    pub tag_list: Vec<String>,
    pub title: String,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "article"]
pub struct UpdateArticle {
    pub author_id: Option<i64>,
    pub description: Option<String>,
    pub favorites_count: Option<i32>,
    pub slug: Option<String>,
    pub tag_list: Option<Vec<String>>,
    pub title: Option<String>,
}
