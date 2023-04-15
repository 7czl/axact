use std::net::SocketAddr;

use axum::{Server, Router};
use axum::routing::get;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let router = Router::new().route("/", get(rootget));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("");
    Server::bind(&addr).serve(router.into_make_service())
        .await.unwrap();
}
async fn rootget() -> &'static str {
    "Hello from Axum"
}