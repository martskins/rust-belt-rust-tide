use crate::models::Article;

pub async fn list_articles(_ctx: tide::Context<()>) -> String {
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
        // .map_err(|e| tide::http::StatusCode::from_u16(500))
        .unwrap()
}
