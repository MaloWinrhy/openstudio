use axum::{
    extract::State,
    Json,
    routing::post,
    Router,
    handler::Handler, // Import Handler trait for with_state
};
use serde::Deserialize;
use std::sync::Arc;
use core::usecases::project::create_project;
use core::repositories::project_repository::ProjectRepository;

#[derive(Deserialize)]
pub struct CreateProjectInput {
    name: String,
    description: String,
}

pub fn project_routes<R: ProjectRepository + Send + Sync + 'static>(repo: Arc<R>) -> Router {
    Router::new()
        .route("/projects", post(handle_create_project::<R>))
        .with_state(repo)
}

async fn handle_create_project<R: ProjectRepository + Send + Sync + 'static>(
    State(repo): State<Arc<R>>,
    Json(payload): Json<CreateProjectInput>,
) -> Result<Json<String>, (axum::http::StatusCode, String)> {
    let project = create_project(&payload.name, &payload.description);

    repo.save(project)
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json("Project created successfully".into()))
}
