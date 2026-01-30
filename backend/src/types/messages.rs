use serde::{Deserialize, Serialize};
use tungstenite::{Message, Utf8Bytes};

use crate::types::connection::ClientsV2;
use crate::types::question::Question;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub(crate) enum ClientMessage {
    AnswerQuestion { answer: i32 },
    Start {},
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
pub(crate) enum ServerMessage {
    Exiting(),
    GameStarted(),
    Question(Question),
    ConnectionAttempted(),
}
impl ServerMessage {
    // Send method directly for ServerMessage to remove the need of other functions.
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
