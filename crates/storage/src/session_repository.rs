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

use rusqlite::{params, Connection};
use uuid::Uuid;
use models::Session;

use crate::StorageError;

pub struct SessionRepository<'a> {
    pub conn: &'a Connection,
}

impl<'a> SessionRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn create(&self, session: &Session) -> Result<(), StorageError> {
        let created_str = session
            .created_at
            .format(&time::format_description::well_known::Rfc3339)
            .unwrap();

        self.conn.execute(
            "INSERT INTO sessions (token, user_id, created_at)
             VALUES (?1, ?2, ?3)",
            params![
                session.token.to_string(),
                session.user_id.to_string(),
                created_str
            ],
        )?;

        Ok(())
    }

    pub fn find(&self, token: &Uuid) -> Result<Option<Session>, StorageError> {
        let mut stmt = self.conn.prepare(
            "SELECT token, user_id, created_at FROM sessions WHERE token = ?1",
        )?;

        let mut rows = stmt.query(params![token.to_string()])?;

        if let Some(row) = rows.next()? {
            let created_str: String = row.get(2)?;
            let created_at = time::OffsetDateTime::parse(
                &created_str,
                &time::format_description::well_known::Rfc3339,
            )
            .unwrap();

            Ok(Some(Session {
                token: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                user_id: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap(),
                created_at,
            }))
        } else {
            Ok(None)
        }
    }
}