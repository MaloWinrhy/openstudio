use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use core::usecases::project::create_project;
use core::repositories::project_repository::ProjectRepository;
use core::models::project::Project;
use axum::response::IntoResponse;

#[derive(Deserialize)]
pub struct CreateProjectInput {
    name: String,
    description: String,
}

#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn ProjectRepository + Send + Sync + 'static>,
}

pub fn project_routes() -> Router<AppState> {
    Router::new()
        .route("/projects", post(handle_create_project))
        .route("/projects", get(list_projects))
}

async fn list_projects(
    State(state): State<AppState>
) -> axum::response::Response {
    use axum::body::Body;
    use axum::http::Response;
    match state.repo.list() {
        Ok(projects) => {
            let body = serde_json::to_string(&projects).unwrap();
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap()
        },
        Err(_) => {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("content-type", "application/json")
                .body(Body::from("[]"))
                .unwrap()
        }
    }
}

fn handle_create_project(
    State(state): State<AppState>,
    Json(payload): Json<CreateProjectInput>,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Json<String>, (StatusCode, String)>> + Send>> {
    Box::pin(async move {
        let project = create_project(&payload.name, &payload.description);
        state.repo.save(project)
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        Ok(Json("Project created successfully".into()))
    })
}
