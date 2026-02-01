use crate::types::{client::Client, connection::ClientsV2, db::db};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AccessRequest {
    pub email: String,
}

#[derive(serde::Serialize)]
pub struct AccessResponse {
    pub message: String,
}

pub async fn add_access(
    State(_clients): State<ClientsV2>,
    client: Client,
    Path(room_id): Path<String>,
    Json(payload): Json<AccessRequest>,
) -> impl IntoResponse {
    let pool = db();

    // 1. Verify room ownership
    let room_exists = sqlx::query!("SELECT owner FROM rooms WHERE id = $1", room_id)
        .fetch_optional(pool)
        .await;

    match room_exists {
        Ok(Some(record)) => {
            if record.owner != client.id {
                return (StatusCode::FORBIDDEN, "You are not the owner of this room")
                    .into_response();
            }
        }
        Ok(None) => return (StatusCode::NOT_FOUND, "Room not found").into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }

    // 2. Find user by email
    let target_user = sqlx::query!("SELECT id FROM clients WHERE email = $1", payload.email)
        .fetch_optional(pool)
        .await;

    let target_user_id = match target_user {
        Ok(Some(user)) => user.id,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, "User with this email not found").into_response();
        }
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    };

    if target_user_id == client.id {
        return (
            StatusCode::BAD_REQUEST,
            "Cannot add yourself to access list",
        )
            .into_response();
    }

    // 3. Add access
    let result = sqlx::query!(
        "INSERT INTO room_access (room_id, client_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        room_id,
        target_user_id
    )
    .execute(pool)
    .await;

    match result {
        Ok(_) => (
            StatusCode::OK,
            Json(AccessResponse {
                message: "Access granted".to_string(),
            }),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Failed to add access: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}

pub async fn remove_access(
    State(_clients): State<ClientsV2>,
    client: Client,
    Path(room_id): Path<String>,
    Json(payload): Json<AccessRequest>,
) -> impl IntoResponse {
    let pool = db();

    // 1. Verify room ownership
    let room_exists = sqlx::query!("SELECT owner FROM rooms WHERE id = $1", room_id)
        .fetch_optional(pool)
        .await;

    match room_exists {
        Ok(Some(record)) => {
            if record.owner != client.id {
                return (StatusCode::FORBIDDEN, "You are not the owner of this room")
                    .into_response();
            }
        }
        Ok(None) => return (StatusCode::NOT_FOUND, "Room not found").into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }

    // 2. Find user by email
    let target_user = sqlx::query!("SELECT id FROM clients WHERE email = $1", payload.email)
        .fetch_optional(pool)
        .await;

    let target_user_id = match target_user {
        Ok(Some(user)) => user.id,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, "User with this email not found").into_response();
        }
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    };

    // 3. Remove access
    let result = sqlx::query!(
        "DELETE FROM room_access WHERE room_id = $1 AND client_id = $2",
        room_id,
        target_user_id
    )
    .execute(pool)
    .await;

    match result {
        Ok(_) => (
            StatusCode::OK,
            Json(AccessResponse {
                message: "Access removed".to_string(),
            }),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Failed to remove access: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct AccessedUser {
    pub id: String,
    pub name: Option<String>,
    pub email: Option<String>,
}

pub async fn list_access(
    State(_clients): State<ClientsV2>,
    client: Client,
    Path(room_id): Path<String>,
) -> impl IntoResponse {
    let pool = db();

    // 1. Verify room ownership
    let room_exists = sqlx::query!("SELECT owner FROM rooms WHERE id = $1", room_id)
        .fetch_optional(pool)
        .await;

    match room_exists {
        Ok(Some(record)) => {
            if record.owner != client.id {
                return (StatusCode::FORBIDDEN, "You are not the owner of this room")
                    .into_response();
            }
        }
        Ok(None) => return (StatusCode::NOT_FOUND, "Room not found").into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response(),
    }

    // 2. Fetch users with access
    let users = sqlx::query_as::<_, AccessedUser>(
        r#"
        SELECT c.id, c.name, c.email
        FROM room_access ra
        JOIN clients c ON ra.client_id = c.id
        WHERE ra.room_id = $1
        "#,
    )
    .bind(room_id)
    .fetch_all(pool)
    .await;

    match users {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch accessed users: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response()
        }
    }
}
