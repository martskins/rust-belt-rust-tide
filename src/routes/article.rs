use crate::database::Database;
use tide::{EndpointResult, http::StatusCode};
use crate::database::Querier;

pub async fn list_articles(ctx: tide::Context<Database>) -> EndpointResult<String> {
    let db: &Database = ctx.state();
    let conn = db.get_conn();
    let articles = db.articles(conn).find_all()
        .map_err(|e| tide::Error::from(StatusCode::INTERNAL_SERVER_ERROR))?;

    serde_json::to_string(&articles)
        .map_err(|_e| tide::Error::from(StatusCode::INTERNAL_SERVER_ERROR))
}

pub async fn get_article(ctx: tide::Context<Database>) -> EndpointResult<String> {
    let id: i64 = ctx.param("id").unwrap();
    let db: &Database = ctx.state();
    let conn = db.get_conn();
    let article = db.articles(conn).get(id)
        .map_err(|e| tide::Error::from(StatusCode::INTERNAL_SERVER_ERROR))?;

    serde_json::to_string(&article)
        .map_err(|_e| tide::Error::from(StatusCode::INTERNAL_SERVER_ERROR))
}
