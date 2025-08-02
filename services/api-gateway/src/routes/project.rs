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
use uuid;
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
    // --- ROUTES CRUD Project ---
    Router::new()
        .route("/projects", post(handle_create_project))
        .route("/projects", get(list_projects))
        // GET /projects/:id → récupérer un projet par son id
        .route("/projects/:id", get(get_project_by_id))
        // DELETE /projects/:id → supprimer un projet par son id
        // .route("/projects/:id", axum::routing::delete(delete_project_by_id))
        // PUT /projects/:id → mettre à jour un projet par son id
        // .route("/projects/:id", axum::routing::put(update_project_by_id))
}

// Handler GET /projects/:id
// Récupère un projet par son UUID, retourne 404 si non trouvé
async fn get_project_by_id(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> axum::response::Response {
    use axum::body::Body;
    use axum::http::Response;
    match state.repo.get_by_id(id) {
        Ok(Some(project)) => {
            let body = serde_json::to_string(&project).unwrap();
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap()
        },
        Ok(None) => {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Project not found"))
                .unwrap()
        },
        Err(_) => {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Internal server error"))
                .unwrap()
        }
    }
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
