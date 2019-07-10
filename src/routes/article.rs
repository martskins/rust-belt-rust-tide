use crate::models::Article;
use tide::{EndpointResult, http::StatusCode};

pub async fn list_articles(_ctx: tide::Context<()>) -> EndpointResult<String> {
    let articles = vec![Article {
        id: 1,
        slug: "slug-text".to_string(),
        title: "title-text".to_string(),
        description: "description-text".to_string(),
        tag_list: vec![],
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
        favorited: false,
        favorites_count: 0,
        author_id: 1,
    }];

    serde_json::to_string(&articles)
        .map_err(|_e| tide::Error::from(StatusCode::INTERNAL_SERVER_ERROR))
}

pub async fn get_article(ctx: tide::Context<()>) -> EndpointResult<String> {
    let id: i64 = ctx.param("id").unwrap();
    let article = Article {
        id: id,
        slug: "slug-text".to_string(),
        title: "title-text".to_string(),
        description: "description-text".to_string(),
        tag_list: vec![],
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
        favorited: false,
        favorites_count: 0,
        author_id: 1,
    };

    serde_json::to_string(&article)
        .map_err(|_e| tide::Error::from(StatusCode::INTERNAL_SERVER_ERROR))
}
