use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ClientMessage {
    Join { username: String, room: String },
    Chat { text: String },
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ServerMessage {
    Chat { username: String, text: String },
    UserJoined { username: String },
    UserLeft { username: String },
    History { messages: Vec<ServerMessage> },
}
