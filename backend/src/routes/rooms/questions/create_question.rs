use crate::types::{client::Client, connection::ClientsV2, db::db};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::Row;

#[derive(Deserialize)]
pub struct CreateQuestionPayload {
    pub question: String,
    pub img_url: Option<String>,
    pub video_url: Option<String>,
    pub answers: Vec<String>,
    pub correct: i32,
    pub difficulty: Option<i32>,
}

pub async fn create_question(
    State(_clients): State<ClientsV2>,
    Path(room_id): Path<String>,
    client: Client,
    Json(payload): Json<CreateQuestionPayload>,
) -> impl IntoResponse {
    let owner_id = client.id;

    // Check ownership
    let room_check = sqlx::query(
        r#"
        SELECT 1 
        FROM rooms 
        WHERE id = $1 
        AND (
            owner = $2 
            OR EXISTS (
                SELECT 1 
                FROM room_access 
                WHERE room_id = rooms.id 
                AND client_id = $2
            )
        )
        "#,
    )
    .bind(&room_id)
    .bind(owner_id)
    .fetch_optional(db())
    .await;

    match room_check {
        Ok(Some(_)) => {}
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": "Room not found or access denied" })),
            )
                .into_response();
        }
        Err(e) => {
            tracing::error!("Failed to check room: {e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Database error" })),
            )
                .into_response();
        }
    }

    let query = sqlx::query("INSERT INTO questions (room_id, question, img_url, video_url, answers, correct, difficulty) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id")
        .bind(room_id)
        .bind(payload.question)
        .bind(payload.img_url)
        .bind(payload.video_url)
        .bind(payload.answers)
        .bind(payload.correct)
        .bind(payload.difficulty)
        .fetch_one(db())
        .await;

    match query {
        Ok(row) => {
            let id: String = row.get("id");
            (StatusCode::CREATED, Json(serde_json::json!({ "id": id }))).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to create question: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Database error" })),
            )
                .into_response()
        }
    }
}
