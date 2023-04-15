use axum::extract::State;
use axum::routing::get;
use axum::{Router, Server};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use sysinfo::{CpuExt, System, SystemExt};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let router = Router::new()
        .route("/", get(cpus_get)).with_state(AppState {
        sys: Arc::new(Mutex::new(System::new())),
    }).route("/", rootget);
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

async fn rootget() -> &'static str {
    "Hello"
}

async fn cpus_get(State(state): State<AppState>) -> String {
    use std::fmt::Write;
    let mut s = String::new();
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();
    for (index, cpu) in sys.cpus().iter().enumerate() {
        let i = index + 1;
        let usage = cpu.cpu_usage();
        writeln!(&mut s, "CPU {i} {usage} ").unwrap();
    }
    s
}
