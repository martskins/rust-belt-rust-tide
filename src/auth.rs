use serde::{Deserialize, Serialize};

pub const JWT_SECRET: &str = "SECRET";

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
