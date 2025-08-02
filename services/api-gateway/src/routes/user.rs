use jsonwebtoken::decode;
use axum::response::IntoResponse;
use axum::{extract::State, http::StatusCode, routing::{get, post}, Json, Router};
use axum_extra::extract::TypedHeader;
use headers::{authorization::Bearer, Authorization};
const JWT_SECRET: &str = "supersecretkey";


use serde::Deserialize;
use std::sync::Arc;
use openstudio_core::models::user::User;
use argon2::{Argon2, PasswordHasher};
use rand::rngs::OsRng;
use argon2::password_hash::{SaltString, PasswordHash, PasswordVerifier};
use uuid;
use chrono::Utc;
use openstudio_core::repositories::in_memory_user::InMemoryUserRepo;

#[derive(Deserialize)]
pub struct CreateUserInput {
    pub username: String,
    pub email: String,
    pub password: String,
}
#[derive(serde::Deserialize)]
struct Claims {
    sub: String,
    email: String,
    exp: usize,
}
#[derive(Clone)]
pub struct UserState {
    pub repo: Arc<InMemoryUserRepo>,
}

pub fn user_routes() -> Router<UserState> {
    Router::new()
        .route("/users", post(create_user))
        .route("/register", post(create_user))
        .route("/users", get(list_users_authenticated))
        .route("/users/{id}", get(get_user_by_id_authenticated))
}

async fn create_user(
    State(state): State<UserState>,
    Json(input): Json<CreateUserInput>,
) -> axum::response::Response {
    use axum::body::Body;
    use axum::http::Response;
    // Hash le mot de passe
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(input.password.as_bytes(), &salt).unwrap().to_string();
    let user = User {
        id: uuid::Uuid::new_v4(),
        username: input.username,
        email: input.email,
        password: password_hash,
        created_at: Utc::now(),
    };
    state.repo.save_user(user);
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User created"))
        .unwrap()
}

async fn list_users_authenticated(
    State(state): State<UserState>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> Result<axum::Json<Vec<User>>, StatusCode> {
    let _ = decode::<Claims>(
        bearer.token(),
        &jsonwebtoken::DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &jsonwebtoken::Validation::default(),
    ).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let users = state.repo.list_users();
    Ok(axum::Json(users))
}

async fn get_user_by_id_authenticated(
    State(state): State<UserState>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
) -> Result<axum::Json<serde_json::Value>, StatusCode> {
    let _ = decode::<Claims>(
        bearer.token(),
        &jsonwebtoken::DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &jsonwebtoken::Validation::default(),
    ).map_err(|_| StatusCode::UNAUTHORIZED)?;
    match state.repo.get_user(id) {
        Some(user) => Ok(axum::Json(serde_json::to_value(user).unwrap())),
        None => Err(StatusCode::NOT_FOUND),
    }
}
