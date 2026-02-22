//! Board Repository
//!
//! Developer Notes:
//! - Handles persistence for `Board` data.
//! - All operations return `StorageError`, not raw rusqlite errors.
//!
//! TODO:
//! - Add filtering and paging in future versions.
//!
//! End Notes:
//! Keeps service layer free of DB error details.

use rusqlite::{params, Connection};
use uuid::Uuid;
use time::OffsetDateTime;

use models::Board;
use crate::StorageError;

pub fn insert_board(
    conn: &Connection,
    board: &Board,
) -> Result<(), StorageError> {
    conn.execute(
        "INSERT INTO boards (id, name, description, created_at) VALUES (?1, ?2, ?3, ?4)",
        params![
            board.id.to_string(),
            board.name,
            board.description,
            board.created_at.format(&time::format_description::well_known::Rfc3339).unwrap()
        ],
    )?;
    Ok(())
}

pub fn get_all(conn: &Connection) -> Result<Vec<Board>, StorageError> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, created_at FROM boards",
    )?;

    let rows = stmt.query_map([], |row| {
        // Parse timestamp from stored string
        let created_str: String = row.get(3)?;
        let created_at = time::OffsetDateTime::parse(
            &created_str,
            &time::format_description::well_known::Rfc3339,
        )
        .unwrap();

        Ok(Board {
            id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
            name: row.get(1)?,
            description: row.get(2)?,
            created_at,
        })
    })?;

    let mut result = Vec::new();
    for r in rows {
        result.push(r?);
    }

    Ok(result)
}