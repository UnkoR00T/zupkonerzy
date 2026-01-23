use axum::{extract::{State, Path}, http::StatusCode, response::IntoResponse, Json};
use crate::types::{client::Client, connection::ClientsV2, db::db};

pub async fn delete_room(
    State(_clients): State<ClientsV2>,
    Path(room_id): Path<String>,
    client: Client,
) -> impl IntoResponse {
    let owner_id = client.id;

    let result = sqlx::query("DELETE FROM rooms WHERE id = $1 AND owner = $2")
        .bind(room_id)
        .bind(owner_id)
        .execute(db())
        .await;

    match result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                return (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "Room not found or access denied" }))).into_response();
            }
            (StatusCode::OK, Json(serde_json::json!({ "message": "Room deleted" }))).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to delete room: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Database error" }))).into_response()
        }
    }
}
