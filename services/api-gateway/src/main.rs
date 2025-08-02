use axum::{Router, serve};
use std::sync::Arc;
use tokio::net::TcpListener;
use core::repositories::in_memory::InMemoryProjectRepo;
mod routes;

// Utilise la struct AppState du module project
use crate::routes::project::AppState;

#[tokio::main]
async fn main() {
    let state = AppState {
        repo: Arc::new(InMemoryProjectRepo::default()),
    };

    let api_routes = routes::project::project_routes();

    let app = api_routes.with_state(state.clone());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 API running at http://127.0.0.1:3000");

    serve(listener, app.into_make_service_with_connect_info::<std::net::SocketAddr>())
        .await
        .unwrap();
}
