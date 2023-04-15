use axum::extract::State;
use axum::response::{IntoResponse, Html};
use axum::routing::get;
use axum::{Router, Server, Json};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use sysinfo::{CpuExt, System, SystemExt};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let router = Router::new()
        .route("/api/cpus", get(cpus_get))
        .with_state(AppState {
            sys: Arc::new(Mutex::new(System::new())),
        })
        .route("/", get(rootget));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("");
    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
}
#[axum::debug_handler]
async fn rootget() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.html").await.unwrap();
    // hot reload for devlopment env
    Html(markup)
}
#[axum::debug_handler]
async fn cpus_get(State(state): State<AppState>) -> impl IntoResponse {
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();
    let v: Vec<f32> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
    Json(v)
}
