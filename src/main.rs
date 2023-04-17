use axum::extract::State;
use axum::http::Response;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::{Json, Router, Server};

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use sysinfo::{CpuExt, System, SystemExt};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    let appstate = AppState::default();
    tracing_subscriber::fmt::init();
    let router = Router::new()
        .route("/api/cpus", get(cpus_get))
        .with_state(appstate.clone())
        .route("/index.css", get(indexcss_fetch))
        .route("/index.mjs", get(indexmjs_fetch))
        .route("/", get(rootget));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));


    tokio::task::spawn_blocking(move ||
    {
        let mut sys = System::new();
        loop {
            sys.refresh_cpu();
            let v = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
            {
                let mut cpus = appstate.cpus.lock().unwrap();
                *cpus = v;
            }
            std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
        }

    });
    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
#[derive(Clone, Default)]
struct AppState {
    cpus: Arc<Mutex<Vec<f32>>>,
}
#[axum::debug_handler]
async fn rootget() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.html").await.unwrap();
    // hot reload for devlopment env
    Html(markup)
}
#[axum::debug_handler]
async fn indexmjs_fetch() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.mjs").await.unwrap();
    Response::builder()
        .header("content-type", "application/javascript")
        .body(markup)
        .unwrap()
}
#[axum::debug_handler]
async fn cpus_get(State(state): State<AppState>) -> impl IntoResponse {
    // let mut sys = state.sys.lock().unwrap();
    let lock_start = std::time::Instant::now();

    let v = state.cpus.lock().unwrap().clone();
    let lock_elapse = lock_start.elapsed().as_micros();
    println!("Lock time: {lock_elapse} micros");
    // fixme: intensive operate. need to do in background.
    // sys.refresh_cpu();
    // let v: Vec<f32> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();
    Json(v)
}
#[axum::debug_handler]
async fn indexcss_fetch() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.css").await.unwrap();
    Response::builder()
        .header("content-type", "text/css;charset=utf-8")
        .body(markup)
        .unwrap()
}