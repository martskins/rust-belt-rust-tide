#![feature(async_await, async_closure)]

#[macro_use]
extern crate diesel;

mod database;
mod models;
mod routes;
mod schema;

use database::Database;

fn main() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();

    let db = Database::establish_connection();
    let mut app = tide::App::with_state(db);

    app.at("/").get(async move |_| "Hello, world!");
    app.at("/articles").get(routes::article::index);
    app.at("/articles/:id").get(routes::article::show);
    app.at("/articles").post(routes::article::create);
    app.at("/articles/:id").put(routes::article::update);
    app.at("/articles/:id").delete(routes::article::delete);

    app.run("127.0.0.1:8000")
}
