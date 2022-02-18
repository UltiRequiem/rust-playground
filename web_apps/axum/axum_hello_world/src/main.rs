use axum::{routing::get, Router, Server};

use std::net::SocketAddr;

mod config;

async fn root() -> &'static str {
    "Hello, world!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], config::PORT));

    println!("Listening on http://{}", addr);

    let builder = Server::bind(&addr);

    builder
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|e| panic!("Failed to initialize the application! {}", e));
}
