use diesel::prelude::*;
use tide::{
    error::ResultExt,
    response::{self, IntoResponse},
    EndpointResult,
};

use crate::database::Database;
use crate::models::{Article, NewArticle};
use crate::schema::article;

pub async fn index(ctx: tide::Context<Database>) -> EndpointResult {
    let db: &Database = ctx.state();
    let conn = db.conn.get().server_err()?;
    let articles: Vec<Article> = article::table.load(&conn).server_err()?;

    Ok(response::json(&articles))
}

pub async fn show(ctx: tide::Context<Database>) -> EndpointResult {
    let id: i64 = ctx.param("id").client_err()?;

    let db: &Database = ctx.state();
    let conn = db.conn.get().server_err()?;
    let article: Article = article::table.find(id).first(&conn).server_err()?;

    Ok(response::json(&article))
}

pub async fn create(mut ctx: tide::Context<Database>) -> EndpointResult {
    let new_article: NewArticle = ctx.body_json().await.client_err()?;

    let db: &Database = ctx.state();
    let conn = db.conn.get().server_err()?;
    let article: Article = diesel::insert_into(article::table)
        .values(&new_article)
        .get_result(&conn)
        .server_err()?;

    Ok(response::json(&article))
}

pub async fn delete(ctx: tide::Context<Database>) -> EndpointResult {
    let id: i64 = ctx.param("id").client_err()?;

    let db: &Database = ctx.state();
    let conn = db.conn.get().server_err()?;
    diesel::delete(article::table.find(id))
        .execute(&conn)
        .server_err()?;

    Ok(().into_response())
}
