use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    types::{claims::JWTClaims, db::db},
    JWT_SECRET,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterPayload {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn register(Json(payload): Json<RegisterPayload>) -> impl IntoResponse {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = match argon2.hash_password(payload.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(e) => {
            tracing::error!("{e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Cant hash password"})),
            );
        }
    };

    let user_id = Uuid::new_v4().to_string();

    if let Err(err) =
        sqlx::query("INSERT INTO clients (id, name, email, password) VALUES ($1, $2, $3, $4);")
            .bind(&user_id)
            .bind(&payload.username)
            .bind(payload.email.to_lowercase())
            .bind(&password_hash)
            .execute(db())
            .await
    {
        tracing::error!("DB Failed: {err}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Database error"})),
        );
    }

    let claims = JWTClaims {
        sub: user_id,
        exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
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
            tracing::warn!("{e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "Failed to create token"})),
            );
        }
    };
    tracing::info!("New account created {}", &payload.email);
    (
        StatusCode::CREATED,
        Json(serde_json::json!({ "token": token })),
    )
}
