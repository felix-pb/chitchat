//! This module is responsible for the `/websocket` endpoint.

use crate::database::Message as ChitChatMessage;
use crate::StateExt;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use futures::{SinkExt, StreamExt};

/// This function builds and returns the router for the `/websocket` endpoint.
pub fn make_router() -> Router {
    Router::new().route("/", get(websocket_handler))
}

/// This function handles the lifecycle of a websocket connection.
///
/// After a successful upgrade, we subscribe to the broadcast channel and await
/// messages in a loop. When a message is received, we serialize it to JSON and
/// forward it to the websocket client, until the client disconnects.
async fn websocket_handler(ws: WebSocketUpgrade, state: StateExt) -> impl IntoResponse {
    ws.on_upgrade(|socket: WebSocket| async move {
        tracing::info!("websocket client connected");
        let (mut sender, mut receiver) = socket.split();
        let mut rx = state.tx.subscribe();
        let mut send_task = tokio::spawn(async move {
            while let Ok(message) = rx.recv().await {
                // Safe unwrap: Message -> JSON serialization cannot fail.
                let json = serde_json::to_string(&message).unwrap();
                if let Err(error) = sender.send(Message::Text(json)).await {
                    // If an error occured, assume the client disconnected and exit
                    // the loop. Unfortunately, `axum::Error` doesn't give us details.
                    tracing::warn!("failed to send websocket message: {}", error);
                    break;
                }
            }
        });
        let mut recv_task = tokio::spawn(async move {
            // Ignore all messages sent by the websocket client until they disconnect.
            while let Some(Ok(message)) = receiver.next().await {
                if let Message::Close(_) = message {
                    tracing::info!("websocket client disconnected");
                    break;
                }
            }
        });
        // If either task exits, abort the other.
        tokio::select! {
            _ = (&mut send_task) => recv_task.abort(),
            _ = (&mut recv_task) => send_task.abort(),
        };
    })
}

/// This function broadcasts a chitchat message to all connected clients.
pub fn broadcast_message(message: ChitChatMessage, state: &StateExt) {
    let count = state.tx.send(message).unwrap_or(0);
    let noun = if count == 1 { "client" } else { "clients" };
    tracing::info!("websocket message sent to {} {}", count, noun);
}
