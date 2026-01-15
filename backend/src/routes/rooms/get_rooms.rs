use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use crate::types::{client::Client, connection::ClientsV2, db::db, room::Room};

pub async fn get_rooms(
    State(_clients): State<ClientsV2>,
    client: Client,
) -> impl IntoResponse {
    let owner_id = match uuid::Uuid::parse_str(&client.id) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "Invalid client ID" }))).into_response(),
    };

    let rooms = sqlx::query_as::<_, Room>("SELECT id::text, name, owner::text FROM rooms WHERE owner = $1")
        .bind(owner_id)
        .fetch_all(db())
        .await;

    match rooms {
        Ok(rooms) => (StatusCode::OK, Json(rooms)).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch rooms: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Database error" }))).into_response()
        }
    }
}
