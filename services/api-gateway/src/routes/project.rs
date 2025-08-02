// --- ROUTER ---
pub fn project_routes() -> axum::Router<AppState> {
    use axum::routing::{get, post};
    axum::Router::new()
        .route("/projects", post(handle_create_project))
        .route("/projects", get(list_projects))
        .route("/projects/{id}", get(get_project_by_id))
        .route("/projects/{id}", axum::routing::delete(delete_project_by_id))
        .route("/projects/{id}", axum::routing::put(update_project_by_id))
}
use axum::{
    extract::{State, FromRequestParts},
    http::{StatusCode, request::Parts},
    Json,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Deserialize;
use std::sync::Arc;
use openstudio_core::usecases::project::create_project;
use openstudio_core::repositories::project_repository::ProjectRepository;
use openstudio_core::models::project::Project;
use openstudio_core::models::project_status;
use uuid;


// --- STRUCTS & STATE ---
#[derive(Clone)]
pub struct AppState {
    pub repo: Arc<dyn ProjectRepository + Send + Sync + 'static>,
}

#[derive(Deserialize)]
pub struct UpdateProjectInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<project_status::ProjectStatus>,
    pub visibility: Option<project_status::Visibility>,
}

#[derive(Deserialize)]
pub struct CreateProjectInput {
    pub name: String,
    pub description: String,
}

// --- AUTH EXTRACTOR ---
pub struct AuthBearer;

impl<S> FromRequestParts<S> for AuthBearer
where
    S: Send + Sync,
{
    type Rejection = axum::response::Response;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(auth_header) = parts.headers.get("authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if let Some(token) = auth_str.strip_prefix("Bearer ") {
                    let key = DecodingKey::from_secret(b"supersecretkey");
                    if decode::<serde_json::Value>(token, &key, &Validation::default()).is_ok() {
                        return Ok(AuthBearer);
                    }
                }
            }
        }
        Err(axum::response::Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(axum::body::Body::from("Unauthorized"))
            .unwrap())
    }
}

// --- HANDLERS ---
async fn delete_project_by_id(
    _auth: AuthBearer,
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> axum::response::Response {
    use axum::body::Body;
    use axum::http::Response;
    match state.repo.delete(id) {
        Ok(true) => Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("Project deleted"))
            .unwrap(),
        Ok(false) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Project not found"))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Internal server error"))
            .unwrap(),
    }
}

async fn update_project_by_id(
    _auth: AuthBearer,
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
    Json(input): Json<UpdateProjectInput>,
) -> axum::response::Response {
    use axum::body::Body;
    use axum::http::Response;
    let existing = match state.repo.get_by_id(id) {
        Ok(Some(p)) => p,
        Ok(None) => {
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Project not found"))
                .unwrap();
        },
        Err(_) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Internal server error"))
                .unwrap();
        }
    };
    let updated = Project {
        id,
        name: input.name.unwrap_or(existing.name),
        description: input.description.unwrap_or(existing.description),
        created_at: existing.created_at,
        visibility: input.visibility.unwrap_or(existing.visibility),
        status: input.status.unwrap_or(existing.status),
    };
    match state.repo.update(updated) {
        Ok(true) => Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("Project updated"))
            .unwrap(),
        Ok(false) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Project not found"))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Internal server error"))
            .unwrap(),
    }
}

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

async fn handle_create_project(
    _auth: AuthBearer,
    State(state): State<AppState>,
    Json(payload): Json<CreateProjectInput>,
) -> axum::response::Response {
    use axum::body::Body;
    use axum::http::Response;
    let project = create_project(&payload.name, &payload.description);
    match state.repo.save(project) {
        Ok(_) => Response::builder()
            .status(StatusCode::CREATED)
            .body(Body::from("Project created successfully"))
            .unwrap(),
        Err(e) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(e.to_string()))
            .unwrap(),
    }
}
