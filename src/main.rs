//! ChitChat – A simple web app to engage in trivial matters, i.e. to chitchat.

mod database;
mod messages;
mod users;
mod websocket;

use crate::database::{Database, Message};
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::routing::get_service;
use axum::{AddExtensionLayer, Router, Server};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::broadcast::Sender;
use tower_http::services::{ServeDir, ServeFile};

/// This struct holds the global state shared across all route handlers
/// and lives for the entire duration of the application.
pub struct State {
    /// Handle to the postgres connection pool.
    db: Database,
    /// Sending-half of the channel used to broadcast messages to all
    /// connected websocket clients.
    tx: Sender<Message>,
}

/// This type alias is used by all route handlers that are fallible.
pub type Result<T> = std::result::Result<T, (StatusCode, String)>;

/// This type alias is used by all route handlers using the global state.
pub type StateExt = Extension<Arc<State>>;

impl State {
    /// This constructor, called once at startup, initializes the global state.
    fn new() -> Self {
        Self {
            db: Database::new(),
            tx: tokio::sync::broadcast::channel(1000).0,
        }
    }
}

/// This function is the entrypoint of the application.
#[tokio::main]
async fn main() {
    // Initialize tracing.
    tracing_subscriber::fmt::init();

    // Initialize the top-level app router.
    let app = make_app_router();

    // Start the hyper server on port 3000.
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("server listening on {}", addr);
    if let Err(error) = Server::bind(&addr).serve(app.into_make_service()).await {
        tracing::error!("fatal server error: {}", error);
    }
}

/// This function builds and returns the top-level app router.
fn make_app_router() -> Router {
    let state = Arc::new(State::new());
    let index = ServeFile::new("static/index.html");
    let assets = ServeDir::new("static");
    Router::new()
        .route("/", get_service(index).handle_error(wrap_500))
        .nest("/static", get_service(assets).handle_error(wrap_500))
        .nest("/messages", messages::make_router())
        .nest("/users", users::make_router())
        .nest("/websocket", websocket::make_router())
        .layer(AddExtensionLayer::new(state))
}

/// This function wraps IO errors when serving static file requests
/// in a "500 Internal Server Error" http response.
///
/// Normally, it should never be called.
async fn wrap_500(error: std::io::Error) -> (StatusCode, &'static str) {
    tracing::error!("INTERNAL_SERVER_ERROR: {}", error);
    (StatusCode::INTERNAL_SERVER_ERROR, "Something's wrong!")
}

/// This function wraps errors when serving API requests
/// in a "400 Bad Request" http response.
pub fn wrap_400(error: String) -> (StatusCode, String) {
    tracing::warn!("BAD_REQUEST: {}", error);
    (StatusCode::BAD_REQUEST, error)
}

#[cfg(test)]
mod tests {
    use crate::database::POSTGRES_URI;
    use axum::Server;
    use reqwest::Client;
    use sqlx::{Connection, PgConnection};
    use std::net::{SocketAddr, TcpListener};
    use std::time::Duration;

    /// Initialize the reqwest client, and the hyper server on a random port.
    ///
    /// Each unit test calls this function, which is why we need to start the
    /// hyper server on a random port. Also, we need to reset both tables because
    /// each unit test assumes that there are no messages and no users.
    pub async fn start_client_and_server() -> (Client, SocketAddr) {
        let mut retries = 0;
        let mut conn = loop {
            match PgConnection::connect(POSTGRES_URI).await {
                Ok(conn) => break conn,
                Err(_) => {
                    retries += 1;
                    if retries == 10 {
                        panic!("Failed to connect to posgres after 10 retries");
                    }
                    tokio::time::sleep(Duration::from_secs(1)).await
                }
            }
        };
        sqlx::query("TRUNCATE messages, users CASCADE")
            .execute(&mut conn)
            .await
            .unwrap();

        let listener = TcpListener::bind("0.0.0.0:0").unwrap();
        let server_addr = listener.local_addr().unwrap();
        let app = crate::make_app_router();
        tokio::spawn(async move {
            Server::from_tcp(listener)
                .unwrap()
                .serve(app.into_make_service())
                .await
                .unwrap()
        });

        (Client::new(), server_addr)
    }
}
