use axum::{Router, serve};
use std::sync::Arc;
use tokio::net::TcpListener;
use core::repositories::in_memory::InMemoryProjectRepo;
use core::repositories::in_memory_issue::InMemoryIssueRepo;
mod routes;

use crate::routes::project::AppState;
use crate::routes::issue::{IssueState, issue_routes};

#[tokio::main]
async fn main() {
    let state = AppState {
        repo: Arc::new(InMemoryProjectRepo::default()),
    };
    let issue_state = IssueState {
        repo: Arc::new(InMemoryIssueRepo::new()),
    };

    let api_routes = routes::project::project_routes().with_state(state.clone());
    let issue_api_routes = issue_routes().with_state(issue_state.clone());

    let app = api_routes.merge(issue_api_routes);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 API running at http://127.0.0.1:3000");

    serve(listener, app.into_make_service_with_connect_info::<std::net::SocketAddr>())
        .await
        .unwrap();
}
