use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::middleware::map_response_with_state;
use axum::response::Response;
use axum::routing;
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::broadcast;
use sqlx::Row;

use crate::protocol::{ClientMessage, ServerMessage};
use crate::state::AppState;

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> Response {
    // Handle WebSocket connections here
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();

    //Vent på at klienten sender et "join" meldingsobjekt med brukernavn
    let (username, room) = match receiver.next().await {
        Some(Ok(Message::Text(text))) => match serde_json::from_str::<ClientMessage>(&text) {
            Ok(ClientMessage::Join { username, room }) => (username, room),
            _ => return,
        },
        _ => return,
    };

    let room = room.trim().to_string();
    if room.is_empty() {
    return;
}

    let tx = {
        let mut rooms = state.rooms.lock().unwrap();
        rooms
            .entry(room.clone())
            .or_insert_with(|| broadcast::channel(100).0)
            .clone()
    };
    let mut rx = tx.subscribe();
    let row = sqlx::query("SELECT username, text FROM messages WHERE room = ? ORDER BY id DESC LIMIT 50")
        .bind(&room)
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();
    let mut history: Vec<ServerMessage> = row
        .iter()
        .map(|row|ServerMessage::Chat { username: row.get("username"), text: row.get("text") })
        .collect();

    history.reverse();

    let json = serde_json::to_string(&ServerMessage::History {  messages: history  }).unwrap();
    let _ = sender.send(Message::Text(json.into())).await;
    let _ = tx.send(ServerMessage::UserJoined {
        username: username.clone(),
    });

    loop {
        tokio::select! {
            msg = receiver.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        match serde_json::from_str::<ClientMessage>(&text) {
                            Ok(ClientMessage::Chat { text }) => {
                                let now = std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs() as i64; 
                                let _ = sqlx::query(
                                    "INSERT INTO messages (room, username, text, created_at) VALUES (?, ?, ?, ?)"
                                ).bind(&room)
                                .bind(&username)
                                .bind(&text)
                                .bind(now)
                                .execute(&state.db)
                                .await; 

                                let _ = tx.send(ServerMessage::Chat {
                                    username: username.clone(),
                                    text,
                                });
                            }
                            _ => {}
                        }
                    }
                    _ => break,
                }
            }
            msg = rx.recv() => {
                match msg {
                    Ok(msg) => {
                        let json = serde_json::to_string(&msg).unwrap();
                        if sender.send(Message::Text(json.into())).await.is_err() {
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
        }
    }

    println!("{username} dro");
    let _ = tx.send(ServerMessage::UserLeft { username });
}
