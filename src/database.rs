//! This module is responsible for the database that stores all messages and users.

use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::sync::atomic::{AtomicU32, Ordering::Relaxed};
use std::time::SystemTime;

/// A timestamp is the number of seconds since 1970-01-01 00:00:00 UTC.
///
/// This number should always be much smaller than 2^53 - 1, which is
/// the biggest integer that can safely be represented in JavaScript.
pub type Timestamp = u64;

/// This type alias is used by database methods that are fallible.
pub type Result<T> = std::result::Result<T, &'static str>;

/// A document ID is a unique 32-bit unsigned integer.
pub type DocumentId = u32;

/// This atomic counter is used to generate unique message IDs.
static NEXT_MESSAGE_ID: AtomicU32 = AtomicU32::new(1);

/// This atomic counter is used to generate unique user IDs.
static NEXT_USER_ID: AtomicU32 = AtomicU32::new(1);

/// This struct represents the chitchat database to store all messages and users.
///
/// It is currently implemented with in-memory data structures, but it could be
/// implemented with a connection pool to real persistent databases without
/// changing most of its public API.
pub struct Database {
    max_history: Option<usize>,
    messages: BTreeSet<Message>,
    users: BTreeSet<User>,
}

/// This struct represents a message document in the database.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Message {
    pub id: DocumentId,
    pub author: DocumentId,
    pub text: String,
    pub created: Timestamp,
    pub modified: Option<Timestamp>,
}

/// This struct represents a user document in the database.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: DocumentId,
    pub password: String,
}

/// This struct contains all the parameters needed to create a message.
#[derive(Deserialize, Serialize)]
pub struct CreateMessageParams {
    pub user: User,
    pub text: String,
}

/// This struct contains all the parameters needed to update a message.
#[derive(Deserialize, Serialize)]
pub struct UpdateMessageParams {
    pub message: DocumentId,
    pub user: User,
    pub text: String,
}

/// This struct contains all the parameters needed to delete a message.
#[derive(Deserialize, Serialize)]
pub struct DeleteMessageParams {
    pub message: DocumentId,
    pub user: User,
}

impl Database {
    /// This constructor initializes an empty database.
    ///
    /// It stores up to `max_history` messages if specified,
    /// or all messages forever if not specified.
    pub fn new(max_history: Option<usize>) -> Self {
        Self {
            max_history,
            messages: BTreeSet::new(),
            users: BTreeSet::new(),
        }
    }

    /// This method creates a new user with a unique ID and a random
    /// password, inserts it in the database, and returns it.
    pub fn create_user(&mut self) -> User {
        let user = User {
            id: NEXT_USER_ID.fetch_add(1, Relaxed),
            password: format!("{:x}", rand::random::<u128>()),
        };
        self.users.insert(user.clone());
        user
    }

    /// This method returns an iterator over all messages sorted in
    /// chronological order.
    pub fn read_messages(&mut self) -> impl Iterator<Item = &Message> {
        self.messages.iter()
    }

    /// This method creates a new message in the database, if the parameters
    /// are valid, and returns it. Otherwise, it returns an error message.
    pub fn create_message(&mut self, params: CreateMessageParams) -> Result<Message> {
        self.authenticate_user(&params.user)?;
        Database::validate_text(&params.text)?;
        let message = Message {
            id: NEXT_MESSAGE_ID.fetch_add(1, Relaxed),
            created: Database::generate_unix_timestamp()?,
            modified: None,
            author: params.user.id,
            text: params.text,
        };
        self.messages.insert(message.clone());
        self.delete_oldest_message_at_max_history();
        Ok(message)
    }

    /// This method updates an existing message in the database, if the parameters
    /// are valid, and returns it. Otherwise, it returns an error message.
    pub fn update_message(&mut self, params: UpdateMessageParams) -> Result<Message> {
        self.authenticate_user(&params.user)?;
        let matched_message = self.validate_authorship(&params.user, params.message)?;
        Database::validate_text(&params.text)?;
        let message = Message {
            id: params.message,
            created: matched_message.created,
            modified: Some(Database::generate_unix_timestamp()?),
            author: params.user.id,
            text: params.text,
        };
        self.messages.replace(message.clone());
        Ok(message)
    }

    /// This method deletes an existing message in the database, if the parameters
    /// are valid, and returns it. Otherwise, it returns an error message.
    pub fn delete_message(&mut self, params: DeleteMessageParams) -> Result<Message> {
        self.authenticate_user(&params.user)?;
        self.validate_authorship(&params.user, params.message)?;
        // Safe unwrap: message is guaranteed to exist from `validate_authorship()`.
        let message = self.messages.take(&params.message).unwrap();
        Ok(message)
    }

    /// This private function generates a timestamp based on the current system time.
    fn generate_unix_timestamp() -> Result<Timestamp> {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => Ok(duration.as_secs()),
            Err(_) => Err("Something is wrong with the server... Please try again!"),
        }
    }

    /// This private method validates that the input user ID exists and that their
    /// password matches the corresponding user in the database.
    fn authenticate_user(&self, user: &User) -> Result<()> {
        let matched_user = self.users.get(&user.id).ok_or("Username doesn't exist")?;
        if matched_user.password != user.password {
            return Err("Password doesn't match");
        }
        Ok(())
    }

    /// This private method validates that the input message ID exists and that the
    /// user matches the author of the corresponding message in the database.
    fn validate_authorship(&self, user: &User, id: DocumentId) -> Result<&Message> {
        let matched_message = self.messages.get(&id).ok_or("Message doesn't exist")?;
        if matched_message.author != user.id {
            return Err("You're not the author!");
        }
        Ok(matched_message)
    }

    /// This private method deletes the oldest message from the database if its maximum
    /// capacity has been reached.
    fn delete_oldest_message_at_max_history(&mut self) {
        if let Some(max) = self.max_history {
            if self.messages.len() > max {
                // `BTreeSet::pop_first()` is unstable but would be really nice here.
                // This is an ugly but correct alternative because `BTreeSet::iter()`
                // visits the values in ascending order.
                if let Some(oldest_message) = self.messages.iter().next() {
                    let message_id = oldest_message.id;
                    self.messages.remove(&message_id);
                }
            }
        }
    }

    /// This private function validates the user input text and can easily be extended
    /// with more validation rules.
    fn validate_text(text: &str) -> Result<()> {
        if text.len() > 100 {
            return Err("Maximum 100 characters please!");
        }
        if !text.is_ascii() {
            return Err("ASCII characters only please!");
        }
        if text.to_ascii_lowercase().contains("fuck") {
            return Err("No swear words please!");
        }
        Ok(())
    }
}

/// This macro implements certain traits such that a `BTreeSet` document is indexed
/// by it's `id` field only rather than all of its fields.
macro_rules! impl_by_id {
    ($struct:ident) => {
        impl Borrow<DocumentId> for $struct {
            fn borrow(&self) -> &DocumentId {
                &self.id
            }
        }

        impl PartialEq for $struct {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }

        impl Eq for $struct {}

        impl PartialOrd for $struct {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.id.partial_cmp(&other.id)
            }
        }

        impl Ord for $struct {
            fn cmp(&self, other: &Self) -> Ordering {
                self.id.cmp(&other.id)
            }
        }
    };
}

impl_by_id!(Message);
impl_by_id!(User);
