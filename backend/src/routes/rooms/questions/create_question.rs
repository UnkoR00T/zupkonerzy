use axum::{extract::{State, Path}, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use sqlx::Row;
use crate::types::{client::Client, connection::ClientsV2, db::db};

#[derive(Deserialize)]
pub struct CreateQuestionPayload {
    pub question: String,
    pub img_url: Option<String>,
    pub video_url: Option<String>,
    pub answers: Vec<String>,
    pub correct: i32,
}

pub async fn create_question(
    State(_clients): State<ClientsV2>,
    Path(room_id): Path<String>,
    client: Client,
    Json(payload): Json<CreateQuestionPayload>,
) -> impl IntoResponse {
    let room_uuid = match uuid::Uuid::parse_str(&room_id) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "Invalid room ID" }))).into_response(),
    };
    let owner_id = match uuid::Uuid::parse_str(&client.id) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "Invalid client ID" }))).into_response(),
    };

    // Check ownership
    let room_check = sqlx::query("SELECT 1 FROM rooms WHERE id = $1 AND owner = $2")
        .bind(room_uuid)
        .bind(owner_id)
        .fetch_optional(db())
        .await;
    
    match room_check {
        Ok(Some(_)) => {},
        Ok(None) => return (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "Room not found or access denied" }))).into_response(),
        Err(e) => {
             tracing::error!("Failed to check room: {e}");
             return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Database error" }))).into_response();
        }
    }

    let query = sqlx::query("INSERT INTO questions (room_id, question, img_url, video_url, answers, correct) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id")
        .bind(room_uuid)
        .bind(payload.question)
        .bind(payload.img_url)
        .bind(payload.video_url)
        .bind(payload.answers)
        .bind(payload.correct)
        .fetch_one(db())
        .await;

    match query {
        Ok(row) => {
            let id: uuid::Uuid = row.get("id");
            (StatusCode::CREATED, Json(serde_json::json!({ "id": id.to_string() }))).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to create question: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Database error" }))).into_response()
        }
    }
}
