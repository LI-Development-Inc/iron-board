use rusqlite::{params, Connection, Result};
use uuid::Uuid;
use time::OffsetDateTime;

use models::Board;

pub fn insert_board(conn: &Connection, board: &Board) -> Result<()> {
    conn.execute(
        "INSERT INTO boards VALUES (?1, ?2, ?3, ?4)",
        params![
            board.id.to_string(),
            board.name,
            board.description,
            board.created_at.to_string()
        ],
    )?;
    Ok(())
}

pub fn get_all(conn: &Connection) -> Result<Vec<Board>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, created_at FROM boards",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(Board {
            id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
            name: row.get(1)?,
            description: row.get(2)?,
            created_at: OffsetDateTime::now_utc(),
        })
    })?;

    let mut result = Vec::new();
    for r in rows {
        result.push(r?);
    }

    Ok(result)
}