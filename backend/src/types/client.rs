use crate::types::connection::ClientsV2;
use crate::types::db::db;
use crate::{JWT_SECRET, types::claims::JWTClaims};
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use jsonwebtoken::{DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use sqlx::postgres::PgRow;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Client {
    pub id: String,
    name: String,
    token: Option<String>,
}

impl<S> FromRequestParts<S> for Client
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .ok_or((
                StatusCode::UNAUTHORIZED,
                "Missing Authorization header".to_string(),
            ))?
            .to_str()
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    "Invalid Authorization header".to_string(),
                )
            })?;

        let token = auth_header.strip_prefix("Bearer ").unwrap_or(auth_header);

        let client = Client::from_jwt(&token.to_string())
            .await
            .ok_or((StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

        Ok(client)
    }
}

impl Client {
    pub fn from_row(query: PgRow) -> Client {
        Client {
            id: query.get("id"),
            name: query.get("name"),
            token: None,
        }
    }
    pub async fn from_jwt(auth: &String) -> Option<Client> {
        let secret = JWT_SECRET.get().expect("JWT_SECRET not set.");
        let claims = jsonwebtoken::decode::<JWTClaims>(
            auth,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        );
        match claims {
            Ok(jwt) => {
                let client = Client::try_get(jwt.claims.sub).await?;
                Some(client)
            }
            Err(_) => None,
        }
    }
    pub async fn try_get(id: String) -> Option<Client> {
        let query = sqlx::query("SELECT id, name, token FROM public.clients WHERE id = $1")
            .bind(id)
            .fetch_optional(db())
            .await
            .unwrap()?;
        let client = Client {
            id: query.get("id"),
            name: query.get("name"),
            token: query.get("token"),
        };
        Some(client)
    }
}
