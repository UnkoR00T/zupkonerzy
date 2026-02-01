use crate::types::{client::Client, connection::ClientsV2, db::db, room::Room};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

pub async fn get_rooms(State(_clients): State<ClientsV2>, client: Client) -> impl IntoResponse {
    let owner_id = client.id;

    let rooms = sqlx::query_as::<_, Room>(
        r#"
        SELECT DISTINCT r.id, r.name, r.owner::text 
        FROM rooms r
        LEFT JOIN room_access ra ON r.id = ra.room_id
        WHERE r.owner = $1 OR ra.client_id = $1
        "#,
    )
    .bind(owner_id)
    .fetch_all(db())
    .await;

    match rooms {
        Ok(rooms) => (StatusCode::OK, Json(rooms)).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch rooms: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Database error" })),
            )
                .into_response()
        }
    }
}
