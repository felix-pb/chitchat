//! This module is responsible for the database that stores all messages and users.

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use std::time::SystemTime;

/// A timestamp is the number of seconds since 1970-01-01 00:00:00 UTC.
///
/// This number should always be much smaller than 2^53 - 1, which is
/// the biggest integer that can safely be represented in JavaScript.
pub type Timestamp = i64;

/// This type alias is used by database methods that are fallible.
pub type Result<T> = std::result::Result<T, String>;

/// An ID is a unique 32-bit unsigned integer.
pub type Id = i32;

/// This struct represents the chitchat database to store all messages and users.
pub struct Database {
    pool: PgPool,
}

/// This struct represents a message document in the database.
#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub struct Message {
    pub id: Id,
    pub author: Id,
    pub text: String,
    pub created: Timestamp,
    pub modified: Option<Timestamp>,
}

/// This struct represents a user document in the database.
#[derive(Clone, Debug, Deserialize, FromRow, Serialize)]
pub struct User {
    pub id: Id,
    pub password: String,
}

/// This struct contains all the parameters needed to create a message.
#[derive(Deserialize, Serialize)]
pub struct CreateMessage {
    pub user: User,
    pub text: String,
}

/// This struct contains all the parameters needed to update a message.
#[derive(Deserialize, Serialize)]
pub struct UpdateMessage {
    pub message: Id,
    pub user: User,
    pub text: String,
}

/// This struct contains all the parameters needed to delete a message.
#[derive(Deserialize, Serialize)]
pub struct DeleteMessage {
    pub message: Id,
    pub user: User,
}

/// This constant is the connection URI for postgres.
pub const POSTGRES_URI: &str = "postgres://postgres:password@postgres";

impl Database {
    /// This constructor initializes the connection pool for postgres.
    pub fn new() -> Self {
        // Safe unwrap: constant URI is guaranteed to be parsed successfully.
        let pool = PgPool::connect_lazy(POSTGRES_URI).unwrap();
        Self { pool }
    }

    /// This method creates a new user with a unique ID and a random
    /// password, inserts it in the database, and returns it.
    pub async fn create_user(&self) -> Result<User> {
        let password = format!("'{:x}'", rand::random::<u128>());
        let query = format!(
            "INSERT INTO users(password) VALUES ({}) RETURNING *",
            password
        );
        sqlx::query_as(&query)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("Failed to create user: {}", e))
    }

    /// This method returns an iterator over all messages sorted in
    /// chronological order.
    pub async fn read_messages(&self) -> Result<Vec<Message>> {
        let query = "SELECT * FROM messages ORDER BY created ASC";
        sqlx::query_as(query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to read messages: {}", e))
    }

    /// This method creates a new message in the database, if the parameters
    /// are valid, and returns it. Otherwise, it returns an error message.
    pub async fn create_message(&self, params: CreateMessage) -> Result<Message> {
        self.authenticate_user(&params.user).await?;
        Database::validate_text(&params.text)?;
        let author = params.user.id;
        let text = format!("'{}'", params.text);
        let created = Database::generate_unix_timestamp()?;
        let query = format!(
            "INSERT INTO messages(author, text, created) VALUES ({}, {}, {}) RETURNING *",
            author, text, created
        );
        sqlx::query_as(&query)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("Failed to create message: {}", e))
    }

    /// This method updates an existing message in the database, if the parameters
    /// are valid, and returns it. Otherwise, it returns an error message.
    pub async fn update_message(&self, params: UpdateMessage) -> Result<Message> {
        self.authenticate_user(&params.user).await?;
        self.validate_authorship(&params.user, params.message)
            .await?;
        Database::validate_text(&params.text)?;
        let text = format!("'{}'", params.text);
        let modified = Database::generate_unix_timestamp()?;
        let id = params.message;
        let query = format!(
            "UPDATE messages SET text = {}, modified = {} WHERE id = {} RETURNING *",
            text, modified, id
        );
        sqlx::query_as(&query)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("Failed to update message: {}", e))
    }

    /// This method deletes an existing message in the database, if the parameters
    /// are valid, and returns it. Otherwise, it returns an error message.
    pub async fn delete_message(&self, params: DeleteMessage) -> Result<Message> {
        self.authenticate_user(&params.user).await?;
        self.validate_authorship(&params.user, params.message)
            .await?;
        let id = params.message;
        let query = format!("DELETE FROM messages WHERE id = {} RETURNING *", id);
        sqlx::query_as(&query)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| format!("Failed to delete message: {}", e))
    }

    /// This private function generates a timestamp based on the current system time.
    fn generate_unix_timestamp() -> Result<Timestamp> {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => Ok(duration.as_secs() as i64),
            Err(_) => Err("Failed to retrieve server time".to_string()),
        }
    }

    /// This private method validates that the input user ID exists and that their
    /// password matches the corresponding user in the database.
    async fn authenticate_user(&self, user: &User) -> Result<()> {
        let id = user.id;
        let query = format!("SELECT * FROM users WHERE id = {}", id);
        let matched_user = sqlx::query_as::<_, User>(&query)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| "Username doesn't exist".to_string())?;
        if matched_user.password != user.password {
            return Err("Password doesn't match".to_string());
        }
        Ok(())
    }

    /// This private method validates that the input message ID exists and that the
    /// user matches the author of the corresponding message in the database.
    async fn validate_authorship(&self, user: &User, id: Id) -> Result<()> {
        let query = format!("SELECT * FROM messages WHERE id = {}", id);
        let matched_message = sqlx::query_as::<_, Message>(&query)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| "UMessage doesn't exist".to_string())?;
        if matched_message.author != user.id {
            return Err("You're not the author!".to_string());
        }
        Ok(())
    }

    /// This private function validates the user input text and can easily be extended
    /// with more validation rules.
    fn validate_text(text: &str) -> Result<()> {
        if text.len() > 100 {
            return Err("Maximum 100 characters please!".to_string());
        }
        if !text.is_ascii() {
            return Err("ASCII characters only please!".to_string());
        }
        if text.to_ascii_lowercase().contains("fuck") {
            return Err("No swear words please!".to_string());
        }
        Ok(())
    }
}
