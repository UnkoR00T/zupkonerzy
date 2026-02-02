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

    match msg {
        ClientMessage::AnswerQuestion { answer } => {
            if let Some(mut room) = games.get_mut(room_id) {
                let question = match room.current_question.clone() {
                    Some(question) => question,
                    None => return,
                };
                if room.current_marked_answer.is_none()
                    || room.current_marked_answer.unwrap_or(-1) != answer
                {
                    room.current_marked_answer = Some(answer);
                    ServerMessage::MarkQuestion(answer).broadcast_room(clients, room_id);
                } else {
                    room.current_marked_answer = None;
                    ServerMessage::FinalAnswer {
                        correct: question.correct,
                        marked: answer,
                    }
                    .broadcast_room(clients, room_id);
                }
            }
        }
        ClientMessage::RerollQuestion {} => {
            if let Some(mut room) = games.get_mut(room_id) {
                if let Some(question) =
                    Question::get_random_question(room.current_question_number).await
                {
                    room.current_question = Some(question.clone());
                    ServerMessage::Question {
                        question: question.strip_answer(),
                        question_number: room.current_question_number,
                    }
                    .broadcast_room(clients, room_id);
                }
            }
        }
        ClientMessage::NextQuestion {} => {
            if let Some(mut room) = games.get_mut(room_id) {
                room.current_question_number += 1;
                room.current_marked_answer = None;
                if let Some(question) =
                    Question::get_random_question(room.current_question_number).await
                {
                    room.current_question = Some(question.clone());
                    ServerMessage::Question {
                        question: question.strip_answer(),
                        question_number: room.current_question_number,
                    }
                    .broadcast_room(clients, room_id);
                }
            }
        }
        ClientMessage::SwitchLadder {} => {
            if let Some(mut room) = games.get_mut(room_id) {
                room.ladder = !room.ladder;
                ServerMessage::SwitchLadder(room.ladder).broadcast_room(clients, room_id);
            }
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
                ServerMessage::Question {
                    question: question.strip_answer(),
                    question_number: 1,
                }
                .broadcast_room(clients, room_id);
            } else {
                info!("No questions found!");
            }
        }
    }
}
