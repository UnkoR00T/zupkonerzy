use std::sync::Arc;

use dashmap::DashMap;
use tokio::sync::{mpsc, oneshot};
use tungstenite::Message;

use crate::types::{client::Client, helper_types::ResponseData};

// Pending acks mapping UUID as String > Sender with ResponseData
pub type PendingRequests = Arc<DashMap<String, oneshot::Sender<ResponseData>>>;

// Client connection type with tx to sending messages directly and pending acks.
#[derive(Debug)]
pub struct ClientConnection {
    pub tx: mpsc::UnboundedSender<Message>,
    pub pending: PendingRequests,
    pub _db_client: Client,
}

// Client mapping UUID as String > ClientConnection
pub type RoomClients = DashMap<String, ClientConnection>;
pub type ClientsV2 = Arc<DashMap<String, RoomClients>>;
