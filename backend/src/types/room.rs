use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Room {
    id: String,
    name: String,
    owner: String,
}
#[derive(Serialize, Deserialize, FromRow)]
pub struct WebsocketRoom {
    id: String,
    room: Room,
}
