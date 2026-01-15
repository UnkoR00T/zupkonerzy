use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Question {
    question: String,
    answers: Vec<String>,
    correct: i32,
}
