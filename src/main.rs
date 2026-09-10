use tokio::net::TcpListener;
use axum::{Router, routing::get}; 
use tower_http::services::ServeDir;
use axum::extract::ws::{WebSocket, WebSocketUpgrade};
use axum::response::Response;


async fn hello() -> &'static str {
    "Hello, World!"
}

async fn ws_handler(ws: WebSocketUpgrade) ->  Response {
    // Handle WebSocket connections here
    ws.on_upgrade(handel_socket)


} 

async fn handel_socket(mut socket: WebSocket) {
     loop {
         match socket.recv().await{
            Some(Ok(msg)) => {let _ = socket.send(msg).await;}
            Some(Err(_)) => break,
            None => break,
            }
     }
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await.unwrap();

    let app: Router = Router::new()
    .route("/hello", get(hello))
    .route("/ws", get(ws_handler))
    .fallback_service(ServeDir::new("static"));

    println!("lytter på http://{addr}");
    axum::serve(listener, app).await.unwrap(); 

}

