use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::types::{
    client::Client,
    connection::ClientsV2,
    gamestate::Games,
    handle_error::HandleError,
    messages::{ClientMessage, ServerMessage},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct SwitchLadder {}

#[typetag::serde]
#[async_trait]
impl ClientMessage for SwitchLadder {
    async fn handle(
        &self,
        clients: &ClientsV2,
        games: &Games,
        room_id: &str,
        _client_id: &Client,
    ) -> Result<(), HandleError> {
        if let Some(mut room) = games.get_mut(room_id) {
            room.ladder = !room.ladder;
            ServerMessage::SwitchLadder(room.ladder).broadcast_room(clients, room_id);
            Ok(())
        } else {
            Err(HandleError::ServerError(1, String::from("No room found!")))
        }
    }
}
