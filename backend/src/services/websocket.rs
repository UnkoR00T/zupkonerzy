use crate::types::{
    client::Client, connection::ClientsV2, gamestate::Games, messages::ClientMessage,
};
pub async fn handle_message(
    clients: &ClientsV2,
    games: &Games,
    room_id: &str,
    client_id: &Client,
    msg: Box<dyn ClientMessage>,
) {
    if let Err(e) = msg.handle(clients, games, room_id, client_id).await {
        tracing::error!("Error handling message: {:?}", e);
    }
}
