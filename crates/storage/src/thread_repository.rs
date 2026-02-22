use rusqlite::{params, Connection, Result};
use uuid::Uuid;
use time::OffsetDateTime;

use models::Thread;

/// Insert thread.
pub fn insert_thread(conn: &Connection, thread: &Thread) -> Result<()> {
    conn.execute(
        r#"
        INSERT INTO threads (id, board_id, title, created_at)
        VALUES (?1, ?2, ?3, ?4)
        "#,
        params![
            thread.id.to_string(),
            thread.board_id.to_string(),
            thread.title,
            thread.created_at.to_string()
        ],
    )?;
    Ok(())
}

/// Get threads by board.
pub fn get_threads_by_board(
    conn: &Connection,
    board_id: Uuid,
) -> Result<Vec<Thread>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, board_id, title, created_at
        FROM threads
        WHERE board_id = ?1
        ORDER BY created_at DESC
        "#,
    )?;

    let rows = stmt.query_map(params![board_id.to_string()], |row| {
        Ok(Thread {
            id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
            board_id: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap(),
            title: row.get(2)?,
            created_at: OffsetDateTime::parse(
                &row.get::<_, String>(3)?,
                &time::format_description::well_known::Rfc3339,
            )
            .unwrap(),
        })
    })?;

    let mut threads = Vec::new();
    for t in rows {
        threads.push(t?);
    }

    Ok(threads)
}