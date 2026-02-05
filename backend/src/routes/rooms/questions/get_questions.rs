use crate::types::{client::Client, connection::ClientsV2, db::db, question::Question};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetQuestionsQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub difficulty: Option<i32>,
    pub search: Option<String>,
}

pub async fn get_questions(
    State(_clients): State<ClientsV2>,
    Path(room_id): Path<String>,
    Query(query): Query<GetQuestionsQuery>,
    client: Client,
) -> impl IntoResponse {
    let owner_id = client.id;

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

    let page = query.page.unwrap_or(1).max(1);
    let limit = query.limit.unwrap_or(50).max(1).min(100);
    let offset = (page - 1) * limit;

    let mut q_builder = sqlx::QueryBuilder::new(
        "SELECT id, question, img_url, video_url, answers, correct, difficulty FROM questions WHERE room_id = ",
    );
    q_builder.push_bind(&room_id);

    if let Some(diff) = query.difficulty {
        q_builder.push(" AND difficulty = ");
        q_builder.push_bind(diff);
    }

    if let Some(search) = &query.search {
        q_builder.push(" AND question ILIKE ");
        q_builder.push_bind(format!("%{}%", search));
    }

    q_builder.push(" LIMIT ");
    q_builder.push_bind(limit);
    q_builder.push(" OFFSET ");
    q_builder.push_bind(offset);

    let questions_query = q_builder.build_query_as::<Question>();
    let questions_result = questions_query.fetch_all(db()).await;

    // Count query
    let mut count_builder =
        sqlx::QueryBuilder::new("SELECT COUNT(*) FROM questions WHERE room_id = ");
    count_builder.push_bind(&room_id);

    if let Some(diff) = query.difficulty {
        count_builder.push(" AND difficulty = ");
        count_builder.push_bind(diff);
    }

    if let Some(search) = &query.search {
        count_builder.push(" AND question ILIKE ");
        count_builder.push_bind(format!("%{}%", search));
    }

    let count_query = count_builder.build_query_scalar::<i64>();
    let count_result = count_query.fetch_one(db()).await;

    match (questions_result, count_result) {
        (Ok(questions), Ok(total)) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "questions": questions,
                "total": total,
                "page": page,
                "limit": limit
            })),
        )
            .into_response(),
        (Err(e), _) | (_, Err(e)) => {
            tracing::error!("Failed to fetch questions: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": "Database error" })),
            )
                .into_response()
        }
    }
}
