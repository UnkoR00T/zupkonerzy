use crate::types::client::Client;
use crate::types::connection::ClientsV2;
use crate::types::gamestate::Games;
use crate::types::handle_error::HandleError;
use crate::types::messages::{ClientMessage, ServerMessage};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AnswerQuestion {
    pub answer: i32,
}

#[typetag::serde]
#[async_trait]
impl ClientMessage for AnswerQuestion {
    async fn handle(
        &self,
        clients: &ClientsV2,
        games: &Games,
        room_id: &str,
        _client_id: &Client,
    ) -> Result<(), HandleError> {
        if let Some(mut room) = games.get_mut(room_id) {
            let question = match room.current_question.clone() {
                Some(question) => question,
                None => return Ok(()),
            };
            if room.current_marked_answer.is_none()
                || room.current_marked_answer.unwrap_or(-1) != self.answer
            {
                room.current_marked_answer = Some(self.answer);
                ServerMessage::MarkQuestion(self.answer).broadcast_room(clients, room_id);
            } else {
                room.current_marked_answer = None;
                ServerMessage::FinalAnswer {
                    correct: question.correct,
                    marked: self.answer,
                }
                .broadcast_room(clients, room_id);
            }
            Ok(())
        } else {
            Err(HandleError::ServerError(1, String::from("No room found!")))
        }
    }
}
