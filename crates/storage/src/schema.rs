//! Database Schema Initialization
//!
//! Developer Notes:
//! - Contains ONLY DDL statements.
//! - No logic.
//!
//! End Notes:
//! All schema changes must be reflected here.

use rusqlite::{Connection, Result};

pub fn initialize_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS boards (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            description TEXT NOT NULL,
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS threads (
            id TEXT PRIMARY KEY,
            board_id TEXT NOT NULL,
            title TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY(board_id) REFERENCES boards(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS posts (
            id TEXT PRIMARY KEY,
            thread_id TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY(thread_id) REFERENCES threads(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            role TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS sessions (
            token TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
        );
        "#,
    )?;

    Ok(())
}