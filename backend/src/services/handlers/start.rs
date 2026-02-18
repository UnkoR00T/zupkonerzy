use crate::types::client::Client;
use crate::types::connection::ClientsV2;
use crate::types::gamestate::Games;
use crate::types::handle_error::HandleError;
use crate::types::messages::{ClientMessage, ServerMessage};
use crate::types::question::Question;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Serialize, Deserialize, Debug)]
pub struct Start {}

#[typetag::serde]
#[async_trait]
impl ClientMessage for Start {
    async fn handle(
        &self,
        clients: &ClientsV2,
        games: &Games,
        room_id: &str,
        _client_id: &Client,
    ) -> Result<(), HandleError> {
        info!("Starting game in room {}", room_id);
        ServerMessage::GameStarted().broadcast_room(clients, room_id);
        if let Some(question) = Question::get_random_question(1, &Vec::new()).await {
            if let Some(mut room) = games.get_mut(room_id) {
                room.started = true;
                room.current_question_number = 1;
                room.current_question = Some(question.clone());
                room.used_questions.clear();
                room.used_questions.push(question.id.clone());
            }

            ServerMessage::Question {
                question: question.strip_answer(),
                question_number: 1,
            }
            .broadcast_room(clients, room_id);
            Ok(())
        } else {
            info!("No questions found!");
            Err(HandleError::ServerError(
                1,
                String::from("No questions found!"),
            ))
        }
    }
}
