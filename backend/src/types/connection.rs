use tungstenite::Message;
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

use crate::types::{client::Client, helper_types::ResponseData};

// Client connection type with tx to sending messages directly and pending acks.

pub struct ClientConnection {
    pub tx: mpsc::UnboundedSender<Message>,
    pub pending: Arc<DashMap<String, oneshot::Sender<ResponseData>>>,
    pub _db_client: Client,
}
// Client mapping UUID as String > ClientConnection
pub type RoomClients = DashMap<String, ClientConnection>;
pub type ClientsV2 = Arc<DashMap<String, RoomClients>>;
