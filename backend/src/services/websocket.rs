use tracing::info;

use crate::types::{
    client::Client,
    connection::ClientsV2,
    gamestate::Games,
    messages::{ClientMessage, ServerMessage},
    question::Question,
};

pub async fn handle_message(
    clients: &ClientsV2,
    games: &Games,
    room_id: &str,
    client_id: &Client,
    msg: ClientMessage,
) {
    info!("Handling message from {}: {:?}", client_id.id, msg);
    // Use variables to silence unused warnings for now
    let _ = clients;
    let _ = games;
    let _ = room_id;

    match msg {
        ClientMessage::AnswerQuestion { answer } => {
            info!("Received answer: {}", answer);
        }
        ClientMessage::Start {} => {
            info!("Starting game in room {}", room_id);

            // Broadcast GameStarted
            ServerMessage::GameStarted().broadcast_room(clients, room_id);

            // Fetch Question
            if let Some(question) = Question::get_random_question(1).await {
                // Update State
                if let Some(mut room) = games.get_mut(room_id) {
                    room.started = true;
                    room.current_question_number = 1;
                    room.current_question = Some(question.clone());
                }

                // Broadcast Question
                ServerMessage::Question(question).broadcast_room(clients, room_id);
            } else {
                info!("No questions found!");
            }
        }
    }
}
