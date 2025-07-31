use axum::{Router, routing::post};
use std::sync::Arc;
use tokio::net::TcpListener;
mod routes;
use core::repositories::in_memory::InMemoryProjectRepo;

#[tokio::main]
async fn main() {
    let repo = Arc::new(InMemoryProjectRepo::default());

    let api_routes = routes::project::project_routes(repo);

let app = Router::new()
    .merge(api_routes);


    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 API running at http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
