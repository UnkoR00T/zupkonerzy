use serde::{Deserialize, Serialize};

use crate::types::messages::ServerMessage;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ResponseData {
    Error { error: String },
}

// Ack Message type is just a ServerMessage type with additional field of id.
#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct AckMessage {
    pub id: String,
    #[serde(flatten)]
    pub message: ServerMessage,
}
