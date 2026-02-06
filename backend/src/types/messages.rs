use serde::{Deserialize, Serialize};
use tungstenite::{Message, Utf8Bytes};

use crate::types::client::Client;
use crate::types::connection::ClientsV2;
use crate::types::gamestate::Games;
use crate::types::handle_error::HandleError;
use crate::types::question::Question;
use async_trait::async_trait;

#[typetag::serde(tag = "type", content = "data")]
#[async_trait]
pub(crate) trait ClientMessage: Send + Sync {
    async fn handle(
        &self,
        clients: &ClientsV2,
        games: &Games,
        room_id: &str,
        client_id: &Client,
    ) -> Result<(), HandleError>;
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
pub(crate) enum ServerMessage {
    Exiting(),
    GameStarted(),
    Question {
        question: Question,
        question_number: i32,
    },
    MarkQuestion(i32),
    SwitchLadder(bool),
    FinalAnswer {
        correct: i32,
        marked: i32,
    },
    HelperUsed(i32),
    ConnectionAttempted(),
}
impl ServerMessage {
    pub async fn send(self, clients: &ClientsV2, target: &str) -> Result<(), String> {
        let json: Utf8Bytes = serde_json::to_string(&self).unwrap().into();
        for room in clients.iter() {
            if let Some(client) = room.value().get(target) {
                if client.tx.send(Message::Text(json.clone())).is_err() {
                    return Err(String::from("Failed to send message to client"));
                } else {
                    return Ok(());
                }
            }
        }
        Err(String::from("User not found"))
    }

    pub fn broadcast_room(self, clients: &ClientsV2, room_id: &str) {
        let json: Utf8Bytes = serde_json::to_string(&self).unwrap().into();
        if let Some(room) = clients.get(room_id) {
            for client in room.value().iter() {
                let _ = client.value().tx.send(Message::Text(json.clone()));
            }
        }
    }
}
