use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::types::db::db;

#[derive(Serialize, Deserialize, FromRow, Clone, Debug)]
pub struct Question {
    pub id: String,
    pub question: String,
    pub img_url: Option<String>,
    pub video_url: Option<String>,
    pub answers: Vec<String>,
    pub correct: i32,
    pub difficulty: Option<i32>,
    pub fun_fact: Option<String>,
}
impl Question {
    pub async fn get_from_id(id: &String) -> Option<Self> {
        sqlx::query_as::<_, Question>("SELECT * FROM questions WHERE id = $1")
            .bind(id)
            .fetch_one(db())
            .await
            .ok()
    }

    pub async fn get_random_question(target_diff: i32, excluded: &Vec<String>) -> Option<Self> {
        let q1 = sqlx::query_as::<_, Question>(
            "SELECT * FROM questions WHERE difficulty = $1 AND id != ALL($2) ORDER BY RANDOM() LIMIT 1",
        )
        .bind(target_diff)
        .bind(excluded)
        .fetch_optional(db())
        .await
        .ok()
        .flatten();

        if q1.is_some() {
            return q1;
        }

        // Fallback: order by difference from target difficulty
        sqlx::query_as::<_, Question>("SELECT * FROM questions WHERE id != ALL($2) ORDER BY ABS(COALESCE(difficulty, 0) - $1) ASC, RANDOM() LIMIT 1")
            .bind(target_diff)
            .bind(excluded)
            .fetch_optional(db())
            .await
            .ok()
            .flatten()
    }
    pub fn strip_answer(mut self) -> Self {
        self.correct = -1;
        self.fun_fact = None;
        self
    }
}
