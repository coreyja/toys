use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::SinkExt;
use futures_util::StreamExt;
use std::sync::Arc;
use tokio::select;
use tokio::sync::broadcast;

struct AppState {
    tx: Sender,
}

type Sender = broadcast::Sender<String>;

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    loop {
        select! {
            msg = receiver.next() => {
                if let Some(Ok(Message::Text(text))) = msg {
                    println!("Client sent: {}", text);
                    // Broadcast the message to all clients
                    if state.tx.send(text).is_err() {
                        println!("Error broadcasting message");
                    }
                } else {
                    println!("Client disconnected");
                    break;
                }
            }
            msg = rx.recv() => {
                if let Ok(msg) = msg {
                    if sender.send(Message::Text(msg)).await.is_err() {
                        println!("Error sending message to client");
                        break;
                    }
                }
            }
        }
    }
}

async fn ws_input() -> impl IntoResponse {
    axum::response::Html(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>WebSocket Chat</title>
        </head>
        <body>
            <h1>WebSocket Chat</h1>
            <div id="messages"></div>
            <input type="text" id="messageInput" placeholder="Type a message...">
            <button onclick="sendMessage()">Send</button>

            <script>
                const socket = new WebSocket('ws://127.0.0.1:3000/ws');
                const messagesDiv = document.getElementById('messages');
                const messageInput = document.getElementById('messageInput');

                socket.onopen = () => {
                    console.log('WebSocket connected');
                };

                socket.onmessage = (event) => {
                    const message = document.createElement('p');
                    message.textContent = event.data;
                    messagesDiv.appendChild(message);
                };

                function sendMessage() {
                    const message = messageInput.value;
                    if (message) {
                        socket.send(message);
                        messageInput.value = '';
                    }
                }
            </script>
        </body>
        </html>
        "#,
    )
}

#[tokio::main]
async fn main() {
    let (tx, _rx) = broadcast::channel(100);
    let app_state = Arc::new(AppState { tx });

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/", get(ws_input))
        .with_state(app_state);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
