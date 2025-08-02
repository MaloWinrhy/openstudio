use axum::{extract::State, http::StatusCode, routing::{get, post, delete}, Json, Router};
use serde::Deserialize;
use std::sync::Arc;
use openstudio_core::models::user::{ProjectMember, ProjectRole};
use uuid;
use chrono::Utc;
use openstudio_core::repositories::in_memory_user::InMemoryUserRepo;

#[derive(Deserialize)]
pub struct AddMemberInput {
    pub user_id: uuid::Uuid,
    pub project_id: uuid::Uuid,
    pub role: ProjectRole,
}

#[derive(Clone)]
pub struct MemberState {
    pub repo: Arc<InMemoryUserRepo>,
}

pub fn member_routes() -> Router<MemberState> {
    Router::new()
        .route("/members", post(add_member))
        .route("/members", get(list_members))
        .route("/members", delete(remove_member))
}

async fn add_member(
    State(state): State<MemberState>,
    Json(input): Json<AddMemberInput>,
) -> axum::response::Response {
    use axum::body::Body;
    use axum::http::Response;
    let member = ProjectMember {
        user_id: input.user_id,
        project_id: input.project_id,
        role: input.role,
        joined_at: Utc::now(),
    };
    state.repo.add_member(member);
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("Member added"))
        .unwrap()
}

async fn list_members(
    State(state): State<MemberState>,
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
    let members = state.repo.list_members(project_id);
    let body = serde_json::to_string(&members).unwrap();
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap()
}

#[derive(Deserialize)]
struct RemoveMemberInput {
    user_id: uuid::Uuid,
    project_id: uuid::Uuid,
}

async fn remove_member(
    State(state): State<MemberState>,
    Json(input): Json<RemoveMemberInput>,
) -> axum::response::Response {
    use axum::body::Body;
    use axum::http::Response;
    state.repo.remove_member(input.project_id, input.user_id);
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Member removed"))
        .unwrap()
}
