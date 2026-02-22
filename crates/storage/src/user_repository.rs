//! User Repository
//!
//! Developer Notes:
//! - Handles persistence for Users.
//! - No business logic allowed.
//!
//! TODO:
//! - Add pagination for admin listing.
//!
//! End Notes:
//! Pure SQLite repository.

use rusqlite::{params, Connection, Result};
use uuid::Uuid;
use crate::schema;
use models::User;

pub struct UserRepository<'a> {
    pub conn: &'a Connection,
}

impl<'a> UserRepository<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn create(&self, user: &User) -> Result<()> {
        self.conn.execute(
            "INSERT INTO users (id, username, password_hash, role)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                user.id.to_string(),
                user.username,
                user.password_hash,
                user.role.to_string()
            ],
        )?;
        Ok(())
    }

    pub fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, username, password_hash, role FROM users WHERE username = ?1"
        )?;

        let mut rows = stmt.query(params![username])?;

        if let Some(row) = rows.next()? {
            Ok(Some(User {
                id: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
                username: row.get(1)?,
                password_hash: row.get(2)?,
                role: {
                    let role_str: String = row.get(3)?;
                    role_str.parse().unwrap()
                },
            }))
        } else {
            Ok(None)
        }
    }
}