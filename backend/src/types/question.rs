use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::types::db::db;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Question {
    id: String,
    question: String,
    img_url: Option<String>,
    video_url: Option<String>,
    answers: Vec<String>,
    correct: i32,
}
impl Question {
    pub async fn get_from_id(id: &String) -> Option<Self> {
        sqlx::query_as::<_, Question>("SELECT 1 FROM question WHERE id = $1")
            .bind(id)
            .fetch_one(db())
            .await
            .ok()
    }
}
