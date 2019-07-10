use crate::schema::article;
use diesel::*;
use serde::*;

// TODO: It might be worth trying to create an u32 that implements
// diesel::AsExpression<diesel::sql_types::Integer> to use in favorites_count

#[derive(Serialize, Queryable, Identifiable)]
#[table_name = "article"]
pub struct Article {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub tag_list: Vec<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    // pub favorited: bool,
    pub favorites_count: i32,
    pub author_id: i64,
}

#[derive(Deserialize, Insertable)]
#[table_name = "article"]
pub struct NewArticle {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub tag_list: Vec<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub favorites_count: i32,
    pub author_id: i64,
}
