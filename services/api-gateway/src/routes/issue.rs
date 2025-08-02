use axum::{extract::State, http::StatusCode, routing::{get, post}, Json, Router};
use serde::Deserialize;
use std::sync::Arc;
use core::models::issue::{Issue, IssueStatus};
use core::repositories::issue_repository::IssueRepository;
use uuid;
use chrono::Utc;

#[derive(Deserialize)]
pub struct CreateIssueInput {
    pub project_id: uuid::Uuid,
    pub title: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct UpdateIssueInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<IssueStatus>,
}

#[derive(Clone)]
pub struct IssueState {
    pub repo: Arc<dyn IssueRepository + Send + Sync + 'static>,
}

pub fn issue_routes() -> Router<IssueState> {
    Router::new()
        .route("/issues", post(create_issue))
        .route("/issues", get(list_issues_by_project))
        .route("/issues/{id}", get(get_issue_by_id))
        .route("/issues/{id}", axum::routing::put(update_issue_by_id))
        .route("/issues/{id}", axum::routing::delete(delete_issue_by_id))
}

async fn create_issue(
    State(state): State<IssueState>,
    Json(input): Json<CreateIssueInput>,
) -> axum::response::Response {
    use axum::body::Body;
    use axum::http::Response;
    let issue = Issue {
        id: uuid::Uuid::new_v4(),
        project_id: input.project_id,
        title: input.title,
        description: input.description,
        status: IssueStatus::Open,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    match state.repo.save(issue) {
        Ok(_) => Response::builder()
            .status(StatusCode::CREATED)
            .body(Body::from("Issue created"))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Internal server error"))
            .unwrap(),
    }
}

async fn list_issues_by_project(
    State(state): State<IssueState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> axum::response::Response {
    use axum::body::Body;
    use axum::http::Response;
    let project_id = match params.get("project_id") {
        Some(pid) => match uuid::Uuid::parse_str(pid) {
            Ok(id) => id,
            Err(_) => {
                return Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from("Invalid project_id"))
                    .unwrap();
            }
        },
        None => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Missing project_id"))
                .unwrap();
        }
    };
    match state.repo.list_by_project(project_id) {
        Ok(issues) => {
            let body = serde_json::to_string(&issues).unwrap();
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap()
        },
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Internal server error"))
            .unwrap(),
    }
}

async fn get_issue_by_id(
    State(state): State<IssueState>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> axum::response::Response {
    use axum::body::Body;
    use axum::http::Response;
    match state.repo.get_by_id(id) {
        Ok(Some(issue)) => {
            let body = serde_json::to_string(&issue).unwrap();
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap()
        },
        Ok(None) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Issue not found"))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Internal server error"))
            .unwrap(),
    }
}

async fn update_issue_by_id(
    State(state): State<IssueState>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
    Json(input): Json<UpdateIssueInput>,
) -> axum::response::Response {
    use axum::body::Body;
    use axum::http::Response;
    let existing = match state.repo.get_by_id(id) {
        Ok(Some(i)) => i,
        Ok(None) => {
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Issue not found"))
                .unwrap();
        },
        Err(_) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Internal server error"))
                .unwrap();
        }
    };
    let updated = Issue {
        id,
        project_id: existing.project_id,
        title: input.title.unwrap_or(existing.title),
        description: input.description.unwrap_or(existing.description),
        status: input.status.unwrap_or(existing.status),
        created_at: existing.created_at,
        updated_at: Utc::now(),
    };
    match state.repo.update(updated) {
        Ok(true) => Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("Issue updated"))
            .unwrap(),
        Ok(false) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Issue not found"))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Internal server error"))
            .unwrap(),
    }
}

async fn delete_issue_by_id(
    State(state): State<IssueState>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> axum::response::Response {
    use axum::body::Body;
    use axum::http::Response;
    match state.repo.delete(id) {
        Ok(true) => Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("Issue deleted"))
            .unwrap(),
        Ok(false) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Issue not found"))
            .unwrap(),
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Internal server error"))
            .unwrap(),
    }
}
