//! Storage Crate
//!
//! Developer Notes:
//! - This crate is responsible for all database interaction.
//! - It must NOT contain business logic.
//! - It must ONLY perform persistence operations.
//!
//! End Notes:
//! Keep this layer thin and predictable.

pub mod connection;
pub mod schema;
pub mod board_repository;
pub mod thread_repository;
pub mod post_repository;
pub mod user_repository;
pub mod session_repository;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
}

pub use rusqlite::Connection as DbConnection;


/// TESTS
/// 
/// 

#[cfg(test)]
mod tests {
    use super::*;
    use connection::create_connection;
    use schema::initialize_schema;

    #[test]
    fn database_initializes() {
        let conn = create_connection(":memory:").unwrap();
        initialize_schema(&conn).unwrap();
    }
}