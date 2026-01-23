use crate::types::{client::Client, connection::ClientsV2, db::db, question::Question};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

pub async fn get_questions(
    State(_clients): State<ClientsV2>,
    Path(room_id): Path<String>,
    client: Client,
) -> impl IntoResponse {
    let owner_id = client.id;

    let room_check = sqlx::query("SELECT 1 FROM rooms WHERE id = $1 AND owner = $2")
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

    let questions = sqlx::query_as::<_, Question>("SELECT id, question, img_url, video_url, answers, correct, difficulty FROM questions WHERE room_id = $1")
        .bind(room_id)
        .fetch_all(db())
        .await;

    match questions {
        Ok(questions) => (StatusCode::OK, Json(questions)).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch questions: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Database error" })),
            )
                .into_response()
        }
    }
}
