//! ChitChat – A simple web app to engage in trivial matters, i.e. to chitchat.

mod database;
mod messages;
mod users;
mod websocket;

use crate::database::{Database, Message};
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get_service;
use axum::{AddExtensionLayer, Router, Server};
use parking_lot::Mutex;
use std::net::SocketAddr;
use std::sync::Arc;
use structopt::StructOpt;
use tokio::sync::broadcast::Sender;
use tower_http::services::{ServeDir, ServeFile};

/// A simple web app to engage in trivial matters, i.e. to chitchat.
#[derive(StructOpt)]
struct Opt {
    /// The maximum number of messages stored in the database.
    /// By default, if not specified, all messages are stored forever.
    #[structopt(long, short)]
    max_history: Option<usize>,
}

/// This struct holds the global state shared across all route handlers
/// and lives for the entire duration of the application.
pub struct State {
    /// Internal in-memory database used to store all messages and users.
    db: Mutex<Database>,
    /// Sending-half of the channel used to broadcast messages to all
    /// connected websocket clients.
    tx: Sender<Message>,
}

/// This type alias is used by all route handlers using the global state.
pub type StateExt = Extension<Arc<State>>;

impl State {
    /// This constructor, called once at startup, initializes the global state.
    ///
    /// The internal database stores `max_history` messages if specified,
    /// or all messages forever if not specified.
    fn new(max_history: Option<usize>) -> Self {
        Self {
            db: Mutex::new(Database::new(max_history)),
            tx: tokio::sync::broadcast::channel(1000).0,
        }
    }
}

/// This function is the entrypoint of the application.
#[tokio::main]
async fn main() {
    // Initialize the command-line options.
    let opt = Opt::from_args();

    // Initialize tracing.
    tracing_subscriber::fmt::init();

    // Initialize the top-level app router.
    let app = make_app_router(opt.max_history);

    // Start the hyper server on port 3000.
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("server listening on {}", addr);
    if let Err(error) = Server::bind(&addr).serve(app.into_make_service()).await {
        tracing::error!("fatal server error: {}", error);
    }
}

/// This function builds and returns the top-level app router.
fn make_app_router(max_history: Option<usize>) -> Router {
    let state = Arc::new(State::new(max_history));
    let index = ServeFile::new("static/index.html");
    let assets = ServeDir::new("static");
    Router::new()
        .route("/", get_service(index).handle_error(error_handler))
        .nest("/static", get_service(assets).handle_error(error_handler))
        .nest("/messages", messages::make_router())
        .nest("/users", users::make_router())
        .nest("/websocket", websocket::make_router())
        .layer(AddExtensionLayer::new(state))
}

/// This function handles I/O errors when serving static files.
///
/// Normally, it should never be called.
async fn error_handler(error: std::io::Error) -> impl IntoResponse {
    tracing::error!("failed to serve static file: {}", error);
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Something's wrong with our server... We're on it!",
    )
}

#[cfg(test)]
mod tests {
    use axum::Server;
    use reqwest::Client;
    use std::net::{SocketAddr, TcpListener};

    /// Initialize reqwest client, and hyper server on a random port.
    pub fn start_client_and_server() -> (Client, SocketAddr) {
        let listener = TcpListener::bind("0.0.0.0:0").unwrap();
        let server_addr = listener.local_addr().unwrap();
        let app = crate::make_app_router(None);
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
