use crate::protocol::ServerMessage;
use tokio::sync::broadcast;

pub struct AppState {
    pub tx: broadcast::Sender<ServerMessage>,
}
