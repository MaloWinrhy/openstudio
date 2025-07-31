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
}

pub async fn list_projects(
    State(state): State<AppState>,
) -> Json<Vec<Project>> {
    match state.repo.list() {
        Ok(projects) => Json(projects),
        Err(_) => Json(vec![]),
    }
}
async fn handle_create_project(
    State(state): State<AppState>,
    Json(payload): Json<CreateProjectInput>,
) -> Result<Json<String>, (StatusCode, String)> {
    let project = create_project(&payload.name, &payload.description);
    state.repo.save(project)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json("Project created successfully".into()))
}
