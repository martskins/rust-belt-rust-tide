use crate::auth::{Claims, JWT_SECRET};
use crate::database::Database;
use futures::future::BoxFuture;
use jsonwebtoken::Validation;
use tide::http::StatusCode;
use tide::middleware::{Middleware, Next};
use tide::response::{IntoResponse, Response};
use tide::Context;

pub struct AuthMiddleware;

impl AuthMiddleware {
    pub fn new() -> AuthMiddleware {
        AuthMiddleware {}
    }
}

impl Middleware<Database> for AuthMiddleware {
    fn handle<'a>(
        &'a self,
        cx: Context<Database>,
        next: Next<'a, Database>,
    ) -> BoxFuture<'a, Response> {
        Box::pin(async move {
            if cx.uri() == "/auth/token" {
                let response: Response = next.run(cx).await;
                return response;
            }

            let headers = cx.headers().clone();
            let token = headers.get("Authorization");
            if token.is_none() {
                return StatusCode::FORBIDDEN.into_response();
            }

            let token = token.unwrap().to_str().unwrap();
            let token = token.trim_start_matches("Bearer ");

            let res = jsonwebtoken::decode::<Claims>(
                token,
                JWT_SECRET.as_bytes(),
                &Validation::default(),
            );
            if let Err(e) = res {
                eprintln!("{}", e);
                return StatusCode::FORBIDDEN.into_response();
            }

            let response: Response = next.run(cx).await;
            response
        })
    }
}
