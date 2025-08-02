use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::Deserialize;
use std::sync::Arc;
use openstudio_core::models::user::User;
use openstudio_core::repositories::in_memory_user::InMemoryUserRepo;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;

#[derive(Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
struct Claims {
    sub: String,
    email: String,
    exp: usize,
}

#[derive(Clone)]
pub struct AuthState {
    pub repo: Arc<InMemoryUserRepo>,
    pub jwt_secret: String,
}

pub fn auth_routes() -> Router<AuthState> {
    Router::new().route("/login", post(login))
}

async fn login(
    State(state): State<AuthState>,
    Json(input): Json<LoginInput>,
) -> axum::response::Response {
    use axum::body::Body;
    use axum::http::Response;
    // Recherche d'un user par username ET email (simple)
    let user = state
        .repo
        .list_users()
        .into_iter()
        .find(|u| u.username == input.username && u.email == input.email && u.password == input.password);
    if let Some(user) = user {
        let claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            exp: (chrono::Utc::now().timestamp() + 3600) as usize, // 1h
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
        )
        .unwrap();
        let body = serde_json::json!({"token": token});
        Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from("Invalid credentials"))
            .unwrap()
    }
}
