use axum::{extract::{State, Path}, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use crate::types::{client::Client, connection::ClientsV2, db::db};

#[derive(Deserialize)]
pub struct UpdateRoomPayload {
    pub name: String,
}

pub async fn update_room(
    State(_clients): State<ClientsV2>,
    Path(room_id): Path<String>,
    client: Client,
    Json(payload): Json<UpdateRoomPayload>,
) -> impl IntoResponse {
    let owner_id = client.id;

    let result = sqlx::query("UPDATE rooms SET name = $1 WHERE id = $2 AND owner = $3")
        .bind(payload.name)
        .bind(room_id)
        .bind(owner_id)
        .execute(db())
        .await;

    match result {
        Ok(res) => {
            if res.rows_affected() == 0 {
                return (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "Room not found or access denied" }))).into_response();
            }
            (StatusCode::OK, Json(serde_json::json!({ "message": "Room updated" }))).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to update room: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Database error" }))).into_response()
        }
    }
}
