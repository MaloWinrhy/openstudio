use crate::routes::auth::{AuthState, auth_routes};
use crate::routes::member::{MemberState, member_routes};
use axum::{Router, serve};
use std::sync::Arc;
use tokio::net::TcpListener;
use openstudio_core::repositories::in_memory::InMemoryProjectRepo;
use openstudio_core::repositories::in_memory_issue::InMemoryIssueRepo;
mod routes;

use crate::routes::project::AppState;
use crate::routes::issue::{IssueState, issue_routes};
use crate::routes::user::{UserState, user_routes};
use openstudio_core::repositories::in_memory_user::InMemoryUserRepo;

#[tokio::main]
async fn main() {
    let state = AppState {
        repo: Arc::new(InMemoryProjectRepo::default()),
    };
    let issue_state = IssueState {
        repo: Arc::new(InMemoryIssueRepo::new()),
    };
    let user_state = UserState {
        repo: Arc::new(InMemoryUserRepo::new()),
    };

    let api_routes = routes::project::project_routes().with_state(state.clone());
    let issue_api_routes = issue_routes().with_state(issue_state.clone());
    let user_api_routes = user_routes().with_state(user_state.clone());
    let member_state = MemberState {
        repo: user_state.repo.clone(),
    };
    let member_api_routes = member_routes().with_state(member_state.clone());
    let auth_state = AuthState {
        repo: user_state.repo.clone(),
        jwt_secret: "supersecretkey".to_string(),
    };
    let auth_api_routes = auth_routes().with_state(auth_state.clone());

    let app = api_routes
        .merge(issue_api_routes)
        .merge(user_api_routes)
        .merge(member_api_routes)
        .merge(auth_api_routes);

    let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();
    println!("🚀 API running at http://127.0.0.1:3001");

    serve(listener, app.into_make_service_with_connect_info::<std::net::SocketAddr>())
        .await
        .unwrap();
}
