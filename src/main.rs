#![feature(async_await, async_closure)]

#[macro_use]
extern crate diesel;

mod auth;
mod database;
mod models;
mod routes;
mod schema;

use database::Database;

fn main() -> Result<(), std::io::Error> {
    env_logger::init();
    dotenv::dotenv().ok();

    let db = Database::establish_connection();
    let mut app = tide::App::with_state(db);

    app.middleware(tide::middleware::RequestLogger::new());
    app.middleware(routes::middleware::AuthMiddleware::new());

    app.at("/").get(async move |_| "Hello, world!");

    app.at("/articles").get(routes::article::index);
    app.at("/articles/:id").get(routes::article::show);
    app.at("/articles").post(routes::article::create);
    app.at("/articles/:id").put(routes::article::update);
    app.at("/articles/:id").delete(routes::article::delete);

    app.at("/users").get(routes::user::index);
    app.at("/users/:id").get(routes::user::show);
    app.at("/users").post(routes::user::create);
    app.at("/users/:id").put(routes::user::update);
    app.at("/users/:id").delete(routes::user::delete);

    app.at("/auth/token").post(routes::auth::get_token);

    app.run("127.0.0.1:8000")
}
