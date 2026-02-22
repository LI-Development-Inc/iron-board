//! Services Crate
//!
//! Developer Notes:
//! - This is the business logic layer.
//! - All validation rules live here.
//! - All UUID generation happens here.
//! - All timestamps are created here.
//! - This crate may call the storage layer.
//! - This crate MUST NOT contain SQL.
//!
//! End of File Notes:
//! Keep this layer as the system's rule authority.

use uuid::Uuid;
use time::OffsetDateTime;

use models::{Board, Thread, Post, User, Role};
use storage::{
    board_repository,
    thread_repository,
    post_repository,
    user_repository,
    session_repository,
};

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Storage error: {0}")]
    Storage(#[from] storage::StorageError),
}

pub struct ServiceLayer;

impl ServiceLayer {
    // =========================
    // Board Logic
    // =========================

    pub fn create_board(
        conn: &rusqlite::Connection,
        name: String,
        description: String,
    ) -> Result<Board, ServiceError> {
        if name.trim().is_empty() {
            return Err(ServiceError::Validation(
                "Board name cannot be empty".into(),
            ));
        }

        let board = Board {
            id: Uuid::new_v4(),
            name,
            description,
            created_at: OffsetDateTime::now_utc(),
        };

        board_repository::insert_board(conn, &board)?;
        Ok(board)
    }

    pub fn list_boards(
        conn: &rusqlite::Connection,
    ) -> Result<Vec<Board>, ServiceError> {
        Ok(board_repository::get_all(conn)?)
    }

    // =========================
    // Thread Logic
    // =========================

    pub fn create_thread(
        conn: &rusqlite::Connection,
        board_id: Uuid,
        title: String,
    ) -> Result<Thread, ServiceError> {
        if title.trim().is_empty() {
            return Err(ServiceError::Validation(
                "Thread title cannot be empty".into(),
            ));
        }

        let thread = Thread {
            id: Uuid::new_v4(),
            board_id,
            title,
            created_at: OffsetDateTime::now_utc(),
        };

        thread_repository::insert_thread(conn, &thread)?;
        Ok(thread)
    }

    // =========================
    // Post Logic
    // =========================

    pub fn create_post(
        conn: &rusqlite::Connection,
        thread_id: Uuid,
        content: String,
    ) -> Result<Post, ServiceError> {
        if content.trim().is_empty() {
            return Err(ServiceError::Validation(
                "Post content cannot be empty".into(),
            ));
        }

        let post = Post {
            id: Uuid::new_v4(),
            thread_id,
            content,
            created_at: OffsetDateTime::now_utc(),
        };

        post_repository::insert_post(conn, &post)?;
        Ok(post)
    }
}



/// TESTS:
/// 
/// 
#[cfg(test)]
mod tests {
    use super::*;
    use storage::{connection::create_connection, schema::initialize_schema};

    #[test]
    fn board_validation_works() {
        let conn = create_connection(":memory:").unwrap();
        initialize_schema(&conn).unwrap();

        let result = ServiceLayer::create_board(
            &conn,
            "".into(),
            "desc".into(),
        );

        assert!(result.is_err());
    }
}