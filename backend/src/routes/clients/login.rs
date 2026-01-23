use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

use crate::{
    types::{claims::JWTClaims, db::db},
    JWT_SECRET,
};

#[derive(Serialize, Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

pub async fn login(Json(payload): Json<LoginPayload>) -> impl IntoResponse {
    let query = match sqlx::query("SELECT id, password, name FROM clients WHERE email = $1")
        .bind(payload.email.to_lowercase())
        .fetch_optional(db())
        .await
    {
        Ok(q) => q,
        Err(e) => {
            tracing::error!("{e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Database error"})),
            );
        }
    };

    let Some(res) = query else {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Account not found"})),
        );
    };

    let hash = match PasswordHash::new(res.get("password")) {
        Ok(h) => h,
        Err(e) => {
            tracing::error!("{e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to parse password"})),
            );
        }
    };

    let argon2 = Argon2::default();
    if argon2
        .verify_password(payload.password.as_bytes(), &hash)
        .is_err()
    {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Password is not correct."})),
        );
    }

    let id: String = res.get("id");
    let claims = JWTClaims {
        sub: id,
        exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
        iat: Utc::now().timestamp() as usize,
        jti: Uuid::new_v4().to_string(),
    };
    let secret = JWT_SECRET.get().expect("JWT_SECRET not set.");
    let token = match jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ) {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("{e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to create token"})),
            );
        }
    };
    let username: String = res.get("name");
    (
        StatusCode::OK,
        Json(
            serde_json::json!({ "token": token, "account_details": {"username": username, "email": payload.email} }),
        ),
    )
}
