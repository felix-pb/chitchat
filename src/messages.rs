//! This module is responsible for the `/messages` endpoint.

use crate::database::{CreateMessageParams, DeleteMessageParams, Message, UpdateMessageParams};
use crate::websocket::broadcast_message;
use crate::StateExt;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};

/// This type alias is used by local handlers that are fallible.
type Result = std::result::Result<Json<Message>, (StatusCode, &'static str)>;

/// This function builds and returns the router for the `/messages` endpoint.
pub fn make_router() -> Router {
    Router::new().route(
        "/",
        get(read_messages)
            .post(create_message)
            .put(update_message)
            .delete(delete_message),
    )
}

/// This function handles the `GET /messages` requests.
///
/// It retrieves and returns all messages in chronological order. Currently, there's an
/// option to limit the number of messages stored in the internal database. However, it
/// would be nice to augment this endpoint with support for pagination/lazy-loading.
async fn read_messages(state: StateExt) -> Json<Vec<Message>> {
    let all_messages = state.db.lock().read_messages().cloned().collect();
    Json(all_messages)
}

/// This function handles the `POST /messages` requests.
///
/// It attempts to create a new message. If successful, it broadcasts the created
/// message to all connected clients and returns it. Otherwise, it returns a 400.
async fn create_message(params: Json<CreateMessageParams>, state: StateExt) -> Result {
    let created_message = state.db.lock().create_message(params.0).map_err(wrap_400)?;
    broadcast_message(created_message.clone(), &state);
    Ok(Json(created_message))
}

/// This function handles the `PUT /messages` requests.
///
/// It attempts to update an existing message. If successful, it broadcasts the updated
/// message to all connected clients and returns it. Otherwise, it returns a 400.
async fn update_message(params: Json<UpdateMessageParams>, state: StateExt) -> Result {
    let updated_message = state.db.lock().update_message(params.0).map_err(wrap_400)?;
    broadcast_message(updated_message.clone(), &state);
    Ok(Json(updated_message))
}

/// This function handles the `DELETE /messages` requests.
///
/// It attempts to delete an existing message. If successful, it returns the deleted
/// message. Otherwise, it returns a 400.
async fn delete_message(params: Json<DeleteMessageParams>, state: StateExt) -> Result {
    let deleted_message = state.db.lock().delete_message(params.0).map_err(wrap_400)?;
    Ok(Json(deleted_message))
}

/// This function wraps error messages in a "400 Bad Request" http response.
fn wrap_400(error: &'static str) -> (StatusCode, &'static str) {
    (StatusCode::BAD_REQUEST, error)
}

#[cfg(test)]
pub mod tests {
    use crate::database::{CreateMessageParams, DeleteMessageParams, Message, UpdateMessageParams};
    use reqwest::{Client, StatusCode};
    use std::net::SocketAddr;

    const TEXT: &str = "Hello, World!";

    enum Method {
        Delete(DeleteMessageParams),
        Post(CreateMessageParams),
        Put(UpdateMessageParams),
    }

    async fn send(client: &Client, addr: SocketAddr, method: &Method) -> Result<Message, String> {
        let url = format!("http://{}/messages", addr);
        let request = match method {
            Method::Delete(params) => client.delete(&url).json(params),
            Method::Post(params) => client.post(&url).json(params),
            Method::Put(params) => client.put(&url).json(params),
        };
        let response = request.send().await.unwrap();
        match response.status() {
            StatusCode::OK => Ok(response.json().await.unwrap()),
            StatusCode::BAD_REQUEST => Err(response.text().await.unwrap()),
            _ => panic!("unexpected status code"),
        }
    }

    #[tokio::test]
    async fn it_creates_message() {
        let (client, addr) = crate::tests::start_client_and_server();
        let user1 = crate::users::tests::create_user(&client, addr).await;

        let method = Method::Post(CreateMessageParams {
            user: user1.clone(),
            text: TEXT.to_string(),
        });
        let message1 = send(&client, addr, &method).await.unwrap();

        assert_eq!(user1.id, message1.author);
        assert_eq!(TEXT, message1.text);
        assert!(message1.modified.is_none());
    }

    #[tokio::test]
    async fn it_updates_message() {
        let (client, addr) = crate::tests::start_client_and_server();
        let user1 = crate::users::tests::create_user(&client, addr).await;

        let method = Method::Post(CreateMessageParams {
            user: user1.clone(),
            text: TEXT.to_string(),
        });
        let message1 = send(&client, addr, &method).await.unwrap();

        let method = Method::Put(UpdateMessageParams {
            message: message1.id,
            user: user1.clone(),
            text: TEXT.to_string(),
        });
        let updated_message1 = send(&client, addr, &method).await.unwrap();

        assert_eq!(user1.id, updated_message1.author);
        assert_eq!(TEXT, updated_message1.text);
        assert_eq!(message1.created, updated_message1.created);
        assert!(updated_message1.modified.is_some());
    }

    #[tokio::test]
    async fn it_deletes_message() {
        let (client, addr) = crate::tests::start_client_and_server();
        let user1 = crate::users::tests::create_user(&client, addr).await;

        let method = Method::Post(CreateMessageParams {
            user: user1.clone(),
            text: TEXT.to_string(),
        });
        let message1 = send(&client, addr, &method).await.unwrap();

        let method = Method::Delete(DeleteMessageParams {
            message: message1.id,
            user: user1.clone(),
        });
        let deleted = send(&client, addr, &method).await.unwrap();

        assert_eq!(user1.id, deleted.author);
        assert_eq!(TEXT, deleted.text);
        assert_eq!(message1.created, deleted.created);
        assert!(message1.modified.is_none());
    }

    #[tokio::test]
    async fn it_fails_authentication() {
        let (client, addr) = crate::tests::start_client_and_server();
        let mut user1 = crate::users::tests::create_user(&client, addr).await;
        user1.password = "trying-to-hack-you".to_string();

        let method = Method::Post(CreateMessageParams {
            user: user1.clone(),
            text: TEXT.to_string(),
        });
        let error = send(&client, addr, &method).await.unwrap_err();

        assert_eq!("Password doesn't match", error);
    }

    #[tokio::test]
    async fn it_fails_author_validation() {
        let (client, addr) = crate::tests::start_client_and_server();
        let user1 = crate::users::tests::create_user(&client, addr).await;
        let user2 = crate::users::tests::create_user(&client, addr).await;

        let method = Method::Post(CreateMessageParams {
            user: user1.clone(),
            text: TEXT.to_string(),
        });
        let message1 = send(&client, addr, &method).await.unwrap();

        let method = Method::Put(UpdateMessageParams {
            message: message1.id,
            user: user2.clone(),
            text: TEXT.to_string(),
        });
        let error = send(&client, addr, &method).await.unwrap_err();

        assert_eq!("You're not the author!", error);
    }

    #[tokio::test]
    async fn it_fails_text_validation() {
        let (client, addr) = crate::tests::start_client_and_server();
        let user1 = crate::users::tests::create_user(&client, addr).await;

        let method = Method::Post(CreateMessageParams {
            user: user1.clone(),
            text: "is it okay to say 'fuck' in here?".to_string(),
        });
        let error = send(&client, addr, &method).await.unwrap_err();

        assert_eq!("No swear words please!", error);
    }
}
