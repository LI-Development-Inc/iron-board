//! Session Repository
//!
//! Developer Notes:
//! - Handles session persistence.
//! - UUID tokens.
//!
//! TODO:
//! - Add expiration cleanup job.
//!
//! End Notes:
//! No auth logic here.

use rusqlite::{params, Connection, Result};
use uuid::Uuid;
use models::Session;

pub struct SessionRepository<'a> {
    pub conn: &'a Connection,
}

impl<'a> SessionRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn create(&self, session: &Session) -> Result<()> {
        self.conn.execute(
            "INSERT INTO sessions (token, user_id, created_at)
             VALUES (?1, ?2, ?3)",
            params![
                session.token.to_string(),
                session.user_id.to_string(),
                session.created_at.format(
                    &time::format_description::well_known::Rfc3339
                ).unwrap()
            ],
        )?;
        Ok(())
    }

    pub fn find(&self, token: &Uuid) -> Result<Option<Session>> {
        let mut stmt = self.conn.prepare(
            "SELECT token, user_id, created_at FROM sessions WHERE token = ?1"
        )?;

        let mut rows = stmt.query(params![token.to_string()])?;

        if let Some(row) = rows.next()? {
            Ok(Some(Session {
                token: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                user_id: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap(),
                created_at: {
                    let s: String = row.get(2)?;
                    time::OffsetDateTime::parse(
                        &s,
                        &time::format_description::well_known::Rfc3339,
                    ).unwrap()
},
            }))
        } else {
            Ok(None)
        }
    }
}