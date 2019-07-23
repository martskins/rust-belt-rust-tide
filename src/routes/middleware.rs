use crate::database::Database;
use futures::future::BoxFuture;
use tide::http::StatusCode;
use tide::middleware::{Middleware, Next};
use tide::response::{IntoResponse, Response};
use tide::Context;

pub struct AuthMiddleware;

impl AuthMiddleware {
    pub fn new() -> AuthMiddleware {
        AuthMiddleware {}
    }

    fn parse_token(h: &str) -> &str {
        h.trim_start_matches("Bearer ")
    }

    fn validate_token(_h: &str) -> bool {
        true
    }
}

impl Middleware<Database> for AuthMiddleware {
    fn handle<'a>(
        &'a self,
        cx: Context<Database>,
        next: Next<'a, Database>,
    ) -> BoxFuture<'a, Response> {
        Box::pin(async move {
            let headers = cx.headers().clone();
            let token = headers.get("Authorization");
            if token.is_none() {
                return StatusCode::FORBIDDEN.into_response();
            }

            let token = AuthMiddleware::parse_token(token.unwrap().to_str().unwrap());
            if !AuthMiddleware::validate_token(token) {
                return StatusCode::FORBIDDEN.into_response();
            }

            let response: Response = next.run(cx).await;
            response
        })
    }
}
