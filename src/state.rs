use crate::protocol::ServerMessage;
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::sync::broadcast;
use sqlx::SqlitePool;

pub struct AppState {
    pub rooms: Mutex<HashMap<String, broadcast::Sender<ServerMessage>>>,
    pub db: SqlitePool,
}
