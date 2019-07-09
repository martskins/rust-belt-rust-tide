use tide::{EndpointResult, response};

use crate::models::Article;

pub async fn index(_ctx: tide::Context<()>) -> EndpointResult {
    let articles = vec![Article {
        id: 1,
        slug: "slug-text".to_string(),
        title: "title-text".to_string(),
        description: "description-text".to_string(),
        tag_list: vec![],
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
        favorites_count: 0,
        author_id: 1,
    }];

    Ok(response::json(&articles))
}
