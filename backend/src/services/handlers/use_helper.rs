use crate::types::gamestate::Games;
use crate::types::handle_error::HandleError;
use crate::types::messages::{ClientMessage, ServerMessage};
use crate::types::{client::Client, connection::ClientsV2};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct UseHelper {
    pub helper: i32,
}

#[async_trait]
#[typetag::serde]
impl ClientMessage for UseHelper {
    async fn handle(
        &self,
        clients: &ClientsV2,
        games: &Games,
        room_id: &str,
        _client_id: &Client,
    ) -> Result<(), HandleError> {
        if let Some(mut room) = games.get_mut(room_id) {
            if room.helpers[self.helper as usize] {
                room.helpers[self.helper as usize] = false;
                ServerMessage::HelperUsed(self.helper).broadcast_room(clients, room_id);
                Ok(())
            } else {
                Err(HandleError::ServerError(
                    3,
                    String::from("Helper already used"),
                ))
            }
        } else {
            Err(HandleError::ServerError(1, String::from("Room not found")))
        }
    }
}
