#![feature(async_await, async_closure)]

fn main() -> Result<(), std::io::Error> {
    let mut app = tide::App::new();
    app.at("/").get(async move |_| "Hello, world!");
    Ok(app.run("127.0.0.1:8000")?)
}
