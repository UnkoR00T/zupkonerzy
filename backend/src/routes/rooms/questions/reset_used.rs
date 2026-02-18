use crate::types::{client::Client, gamestate::Games};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

pub async fn reset_used_questions(
    State(games): State<Games>,
    Path(room_id): Path<String>,
    _client: Client,
) -> impl IntoResponse {
    if let Some(mut room) = games.get_mut(&room_id) {
        room.used_questions.clear();
        (
            StatusCode::OK,
            Json(json!({ "message": "Used questions reset" })),
        )
            .into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Room not found or no game active" })),
        )
            .into_response()
    }
}
