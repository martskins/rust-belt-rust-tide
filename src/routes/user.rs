use crate::database::Database;
use crate::models::{NewUser, UpdateUser, User};
use crate::schema::user;
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use tide::{
    error::ResultExt,
    response::{self, IntoResponse},
    EndpointResult,
};

pub async fn index(ctx: tide::Context<Database>) -> EndpointResult {
    let db: &Database = ctx.state();
    let conn = db.conn.get().server_err()?;
    let users: Vec<User> = user::table.load(&conn).server_err()?;

    Ok(response::json(&users))
}

pub async fn show(ctx: tide::Context<Database>) -> EndpointResult {
    let id: i64 = ctx.param("id").client_err()?;

    let db: &Database = ctx.state();
    let conn = db.conn.get().server_err()?;
    let user: User = user::table.find(id).first(&conn).server_err()?;

    Ok(response::json(&user))
}

pub async fn create(mut ctx: tide::Context<Database>) -> EndpointResult {
    let mut new_user: NewUser = ctx.body_json().await.client_err()?;
    new_user.password = hash(new_user.password, DEFAULT_COST).server_err()?;

    let db: &Database = ctx.state();
    let conn = db.conn.get().server_err()?;
    let user: User = diesel::insert_into(user::table)
        .values(&new_user)
        .get_result(&conn)
        .server_err()?;

    Ok(response::json(&user))
}

pub async fn update(mut ctx: tide::Context<Database>) -> EndpointResult {
    let id: i64 = ctx.param("id").client_err()?;
    let mut update_user: UpdateUser = ctx.body_json().await.client_err()?;
    if let Some(password) = update_user.password {
        update_user.password = Some(bcrypt::hash(password, DEFAULT_COST).server_err()?);
    }

    let db: &Database = ctx.state();
    let conn = db.conn.get().server_err()?;
    let user: User = diesel::update(user::table.find(id))
        .set(update_user)
        .get_result(&conn)
        .server_err()?;

    Ok(response::json(&user))
}

pub async fn delete(ctx: tide::Context<Database>) -> EndpointResult {
    let id: i64 = ctx.param("id").client_err()?;

    let db: &Database = ctx.state();
    let conn = db.conn.get().server_err()?;
    diesel::delete(user::table.find(id))
        .execute(&conn)
        .server_err()?;

    Ok(().into_response())
}
