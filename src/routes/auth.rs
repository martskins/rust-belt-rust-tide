use crate::auth::{Claims, JWT_SECRET};
use crate::database::Database;
use crate::models::User;
use crate::schema::user::dsl::{self, user as all_users};
use bcrypt;
use diesel::prelude::*;
use jsonwebtoken::{encode, Header};
use serde::{Deserialize, Serialize};
use std::ops::Add;
use tide::{
    error::ResultExt,
    http::StatusCode,
    response::{self, IntoResponse},
    Context, EndpointResult,
};

#[derive(Debug, Deserialize)]
struct Credentials {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct AuthResponse {
    access_token: String,
}

pub async fn get_token(mut ctx: Context<Database>) -> EndpointResult {
    let credentials: Credentials = ctx.body_json().await.client_err()?;
    let db: &Database = ctx.state();
    let conn = db.conn.get().server_err()?;
    let user: User = all_users
        .filter(dsl::email.eq(credentials.email))
        .first(&conn)
        .server_err()?;
    let valid = bcrypt::verify(credentials.password, &user.password).server_err()?;
    if !valid {
        return Ok(StatusCode::UNAUTHORIZED.into_response());
    }

    let exp = chrono::Utc::now()
        .add(chrono::Duration::days(2))
        .timestamp() as usize;

    let claims = Claims {
        sub: user.email,
        exp,
    };
    let access_token = encode(&Header::default(), &claims, JWT_SECRET.as_ref()).server_err()?;
    let res = AuthResponse { access_token };
    Ok(response::json(&res))
}
