use crate::types::client::Client;
use crate::types::connection::ClientsV2;
use crate::types::gamestate::Games;
use crate::types::handle_error::HandleError;
use crate::types::messages::{ClientMessage, ServerMessage};
use crate::types::question::Question;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RerollQuestion {}

#[typetag::serde]
#[async_trait]
impl ClientMessage for RerollQuestion {
    async fn handle(
        &self,
        clients: &ClientsV2,
        games: &Games,
        room_id: &str,
        _client_id: &Client,
    ) -> Result<(), HandleError> {
        let (q_num, used_qs) = if let Some(room) = games.get(room_id) {
            (room.current_question_number, room.used_questions.clone())
        } else {
            return Ok(());
        };

        if let Some(question) = Question::get_random_question(q_num, &used_qs).await {
            if let Some(mut room) = games.get_mut(room_id) {
                room.current_question = Some(question.clone());
                room.used_questions.push(question.id.clone());
                ServerMessage::Question {
                    question: question.strip_answer(),
                    question_number: q_num,
                }
                .broadcast_room(clients, room_id);
            }
        }
        Ok(())
    }
}
