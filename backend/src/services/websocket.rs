use tracing::info;

use crate::types::{client::Client, connection::ClientsV2, messages::ClientMessage};

pub async fn handle_message(clients: &ClientsV2, room_id: &str, client_id: &Client, msg: ClientMessage) {
    info!("Handling, {:?}", msg);
    match msg {
        ClientMessage::AnswerQuestion { answer } => {}
    }
}
