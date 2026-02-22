use rusqlite::{params, Connection, Result};
use uuid::Uuid;
use time::OffsetDateTime;

use models::Post;

/// Insert post.
pub fn insert_post(conn: &Connection, post: &Post) -> Result<()> {
    conn.execute(
        r#"
        INSERT INTO posts (id, thread_id, content, created_at)
        VALUES (?1, ?2, ?3, ?4)
        "#,
        params![
            post.id.to_string(),
            post.thread_id.to_string(),
            post.content,
            post.created_at.to_string()
        ],
    )?;
    Ok(())
}

/// Get posts by thread.
pub fn get_posts_by_thread(
    conn: &Connection,
    thread_id: Uuid,
) -> Result<Vec<Post>> {
    let mut stmt = conn.prepare(
        r#"
        SELECT id, thread_id, content, created_at
        FROM posts
        WHERE thread_id = ?1
        ORDER BY created_at ASC
        "#,
    )?;

    let rows = stmt.query_map(params![thread_id.to_string()], |row| {
        Ok(Post {
            id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
            thread_id: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap(),
            content: row.get(2)?,
            created_at: OffsetDateTime::parse(
                &row.get::<_, String>(3)?,
                &time::format_description::well_known::Rfc3339,
            )
            .unwrap(),
        })
    })?;

    let mut posts = Vec::new();
    for p in rows {
        posts.push(p?);
    }

    Ok(posts)
}