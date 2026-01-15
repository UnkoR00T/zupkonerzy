use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use sqlx::Row;
use crate::types::{client::Client, connection::ClientsV2, db::db};

#[derive(Deserialize)]
pub struct CreateRoomPayload {
    pub name: String,
}

pub async fn create_room(
    State(_clients): State<ClientsV2>,
    client: Client,
    Json(payload): Json<CreateRoomPayload>,
) -> impl IntoResponse {
    let owner_id = match uuid::Uuid::parse_str(&client.id) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "Invalid client ID" }))),
    };

    let query = sqlx::query("INSERT INTO rooms (name, owner) VALUES ($1, $2) RETURNING id")
        .bind(payload.name)
        .bind(owner_id)
        .fetch_one(db())
        .await;

    match query {
        Ok(row) => {
            let id: uuid::Uuid = row.get("id");
            (StatusCode::CREATED, Json(serde_json::json!({ "id": id.to_string() })))
        }
        Err(e) => {
            tracing::error!("Failed to create room: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Database error" })))
        }
    }
}
