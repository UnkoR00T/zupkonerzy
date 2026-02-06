use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::types::{
    client::Client,
    connection::ClientsV2,
    gamestate::Games,
    handle_error::HandleError,
    messages::{ClientMessage, ServerMessage},
};

#[derive(Serialize, Deserialize)]
pub struct ResetHelpers {}

#[async_trait]
#[typetag::serde]
impl ClientMessage for ResetHelpers {
    async fn handle(
        &self,
        clients: &ClientsV2,
        games: &Games,
        room_id: &str,
        _client_id: &Client,
    ) -> Result<(), HandleError> {
        if let Some(mut room) = games.get_mut(room_id) {
            room.helpers = [true, true, true];
            ServerMessage::Helpers([true, true, true]).broadcast_room(clients, room_id);
            Ok(())
        } else {
            Err(HandleError::ServerError(1, String::from("Room not found")))
        }
    }
}
