use crate::protocol::ServerMessage;
use std::sync::Mutex;
use tokio::sync::broadcast;
use std::collections::HashMap;

pub struct AppState {
pub rooms: Mutex<HashMap<String, broadcast::Sender<ServerMessage>>>,    
    
}

