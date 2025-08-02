use argon2::password_hash::{PasswordHash, PasswordVerifier};
use argon2::Argon2;
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::Deserialize;
use std::sync::Arc;
use openstudio_core::models::user::User;
use openstudio_core::repositories::in_memory_user::InMemoryUserRepo;
use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Validation};
use serde::Serialize;

#[derive(Deserialize)]
pub struct LoginInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
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
    Router::new()
        .route("/login", post(login))
        .route("/refresh", post(refresh_token))
}

#[derive(Deserialize)]
pub struct RefreshInput {
    pub refresh_token: String,
}

async fn refresh_token(
    State(state): State<AuthState>,
    Json(input): Json<RefreshInput>,
) -> axum::response::Response {
    use axum::body::Body;
    use axum::http::Response;
    let token_data = decode::<Claims>(
        &input.refresh_token,
        &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &Validation::default(),
    );
    match token_data {
        Ok(data) => {
            // Vérifie l'expiration du refresh token
            let now = chrono::Utc::now().timestamp() as usize;
            if data.claims.exp < now {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .body(Body::from("Refresh token expired"))
                    .unwrap();
            }
            // Génère un nouvel access token (1h)
            let claims = Claims {
                sub: data.claims.sub.clone(),
                email: data.claims.email.clone(),
                exp: (chrono::Utc::now().timestamp() + 3600) as usize,
            };
            let access_token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
            ).unwrap();
            // Génère un nouveau refresh token (7j)
            let refresh_claims = Claims {
                sub: data.claims.sub,
                email: data.claims.email,
                exp: (chrono::Utc::now().timestamp() + 7 * 24 * 3600) as usize,
            };
            let refresh_token = encode(
                &Header::default(),
                &refresh_claims,
                &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
            ).unwrap();
            let body = serde_json::json!({"access_token": access_token, "refresh_token": refresh_token});
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap()
        }
        Err(_) => Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from("Invalid refresh token"))
            .unwrap(),
    }
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
        .find(|u| u.username == input.username && u.email == input.email);
    if let Some(user) = user {
        let parsed_hash = PasswordHash::new(&user.password);
        if let Ok(parsed_hash) = parsed_hash {
            let argon2 = Argon2::default();
            if argon2.verify_password(input.password.as_bytes(), &parsed_hash).is_ok() {
                // Access token (1h)
                let claims = Claims {
                    sub: user.id.to_string(),
                    email: user.email.clone(),
                    exp: (chrono::Utc::now().timestamp() + 3600) as usize,
                };
                let access_token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
                ).unwrap();
                // Refresh token (7j)
                let refresh_claims = Claims {
                    sub: user.id.to_string(),
                    email: user.email.clone(),
                    exp: (chrono::Utc::now().timestamp() + 7 * 24 * 3600) as usize,
                };
                let refresh_token = encode(
                    &Header::default(),
                    &refresh_claims,
                    &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
                ).unwrap();
                let body = serde_json::json!({"access_token": access_token, "refresh_token": refresh_token});
                return Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "application/json")
                    .body(Body::from(body.to_string()))
                    .unwrap();
            }
        }
        Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from("Invalid credentials"))
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from("Invalid credentials"))
            .unwrap()
    }
}
