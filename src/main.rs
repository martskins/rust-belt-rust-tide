#![feature(async_await, async_closure)]

mod models;
mod routes;

fn main() -> Result<(), std::io::Error> {
    let mut app = tide::App::new();

    app.at("/").get(async move |_| "Hello, world!");
    app.at("/articles").get(routes::article::list_articles);
    app.at("/articles/:id").get(routes::article::get_article);

    app.run("127.0.0.1:8000")
}
