use axum::{extract::{State, Path}, http::StatusCode, response::IntoResponse, Json};
use crate::types::{client::Client, connection::ClientsV2, db::db, question::Question};

pub async fn get_questions(
    State(_clients): State<ClientsV2>,
    Path(room_id): Path<String>,
    client: Client,
) -> impl IntoResponse {
    let room_uuid = match uuid::Uuid::parse_str(&room_id) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "Invalid room ID" }))).into_response(),
    };
    let owner_id = match uuid::Uuid::parse_str(&client.id) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "Invalid client ID" }))).into_response(),
    };

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

    let questions = sqlx::query_as::<_, Question>("SELECT id::text, question, img_url, video_url, answers, correct FROM questions WHERE room_id = $1")
        .bind(room_uuid)
        .fetch_all(db())
        .await;

    match questions {
        Ok(questions) => (StatusCode::OK, Json(questions)).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch questions: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Database error" }))).into_response()
        }
    }
}
