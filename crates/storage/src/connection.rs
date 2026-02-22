//! Database Connection Management
//!
//! Developer Notes:
//! - Creates and configures SQLite connections.
//! - Enables foreign keys.
//!
//! End Notes:
//! Centralized DB setup only.

use rusqlite::{Connection, Result};

pub fn create_connection(path: &str) -> Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    Ok(conn)
}