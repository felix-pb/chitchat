//! This module is responsible for the `/users` endpoint.

use crate::database::User;
use crate::StateExt;
use axum::routing::post;
use axum::{Json, Router};

/// This function builds and returns the router for the `/users` endpoint.
pub fn make_router() -> Router {
    Router::new().route("/", post(create_user))
}

/// This function handles the `POST /users` requests.
///
/// It creates and returns a new user with a unique ID and a random password.
async fn create_user(state: StateExt) -> Json<User> {
    let created_user = state.db.lock().create_user();
    Json(created_user)
}

#[cfg(test)]
pub mod tests {
    use crate::database::User;
    use reqwest::{Client, StatusCode};
    use std::net::SocketAddr;

    pub async fn create_user(client: &Client, addr: SocketAddr) -> User {
        let url = format!("http://{}/users", addr);
        let response = client.post(url).send().await.unwrap();
        assert_eq!(StatusCode::OK, response.status());
        response.json().await.unwrap()
    }

    #[tokio::test]
    async fn it_creates_two_users() {
        let (client, addr) = crate::tests::start_client_and_server();

        let user1 = create_user(&client, addr).await;
        let user2 = create_user(&client, addr).await;

        assert!(user1.id < user2.id);
    }
}
