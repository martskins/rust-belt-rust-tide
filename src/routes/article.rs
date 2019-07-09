use diesel::prelude::*;
use tide::{EndpointResult, error::ResultExt, response};

use crate::database::Database;
use crate::models::Article;
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
