#![feature(async_await, async_closure)]

#[macro_use]
extern crate diesel;

mod models;
mod routes;
mod schema;
mod database;

use database::Database;

fn main() -> Result<(), std::io::Error> {
    let mut app = tide::App::with_state(Database::new());

    app.at("/").get(async move |_| "Hello, world!");
    app.at("/articles").get(routes::article::list_articles);
    app.at("/articles/:id").get(routes::article::get_article);

    app.run("127.0.0.1:8000")
}
