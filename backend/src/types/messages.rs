use serde::{Deserialize, Serialize};
use tungstenite::{Message, Utf8Bytes};

use crate::types::connection::ClientsV2;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub(crate) enum ClientMessage {
    AnswerQuestion { answer: i32 },
}
#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
pub(crate) enum ServerMessage {
    Exiting(),
}
impl ServerMessage {
    // Send method directly for ServerMessage to remove the need of other functions.
    pub async fn send(self, clients: &ClientsV2, target: &str) -> Result<(), String> {
        let json: Utf8Bytes = serde_json::to_string(&self).unwrap().into();
        if let Some(client) = clients.get(target) {
            if client.tx.send(Message::Text(json)).is_err() {
                Err(String::from("Failed to send message to client"))
            } else {
                Ok(())
            }
        } else {
            Err(String::from("User not found"))
        }
    }
}
