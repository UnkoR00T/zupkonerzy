use crate::types::question::Question;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameState {
    pub started: bool,
    pub current_question_number: i32,
    pub current_question: Option<Question>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            started: false,
            current_question_number: 0,
            current_question: None,
        }
    }
}

pub type Games = Arc<DashMap<String, GameState>>;
