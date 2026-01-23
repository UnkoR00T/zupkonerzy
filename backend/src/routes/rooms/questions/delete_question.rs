use axum::{extract::{State, Path}, http::StatusCode, response::IntoResponse, Json};
use crate::types::{client::Client, connection::ClientsV2, db::db};

pub async fn delete_question(
    State(_clients): State<ClientsV2>,
    Path((room_id, question_id)): Path<(String, String)>,
    client: Client,
) -> impl IntoResponse {
    let owner_id = client.id;

    // Check ownership of room
    let room_check = sqlx::query("SELECT 1 FROM rooms WHERE id = $1 AND owner = $2")
        .bind(&room_id)
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

    let result = sqlx::query("DELETE FROM questions WHERE id = $1 AND room_id = $2")
        .bind(question_id)
        .bind(room_id)
        .execute(db())
        .await;

    match result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                return (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "Question not found" }))).into_response();
            }
            (StatusCode::OK, Json(serde_json::json!({ "message": "Question deleted" }))).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to delete question: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Database error" }))).into_response()
        }
    }
}
