use axum::{
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast;

use crate::AppState;
use crate::watcher::WatchEvent;

pub async fn handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    let rx = state.tx.subscribe();
    ws.on_upgrade(|socket| forward_events(socket, rx))
}

async fn forward_events(socket: WebSocket, mut rx: broadcast::Receiver<WatchEvent>) {
    let (mut sender, _) = socket.split();

    loop {
        match rx.recv().await {
            Ok(event) => {
                if sender
                    .send(Message::Text(event.to_string().into()))
                    .await
                    .is_err()
                {
                    break;
                }
            }
            Err(broadcast::error::RecvError::Lagged(_)) => continue,
            Err(broadcast::error::RecvError::Closed) => break,
        }
    }
}
