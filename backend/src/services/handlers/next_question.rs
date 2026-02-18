use crate::types::client::Client;
use crate::types::connection::ClientsV2;
use crate::types::gamestate::Games;
use crate::types::handle_error::HandleError;
use crate::types::messages::{ClientMessage, ServerMessage};
use crate::types::question::Question;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NextQuestion {}

#[typetag::serde]
#[async_trait]
impl ClientMessage for NextQuestion {
    async fn handle(
        &self,
        clients: &ClientsV2,
        games: &Games,
        room_id: &str,
        _client_id: &Client,
    ) -> Result<(), HandleError> {
        let (next_q_num, used_qs) = if let Some(mut room) = games.get_mut(room_id) {
            room.current_question_number += 1;
            room.current_marked_answer = None;
            (room.current_question_number, room.used_questions.clone())
        } else {
            return Ok(());
        };

        if let Some(question) = Question::get_random_question(next_q_num, &used_qs).await {
            if let Some(mut room) = games.get_mut(room_id) {
                room.current_question = Some(question.clone());
                room.used_questions.push(question.id.clone());
                ServerMessage::Question {
                    question: question.strip_answer(),
                    question_number: next_q_num,
                }
                .broadcast_room(clients, room_id);
            }
        }
        Ok(())
    }
}
