//! Models Crate
//!
//! Developer Notes:
//! - This crate defines all domain entities.
//! - It contains NO business logic.
//! - It contains NO database logic.
//! - It contains NO HTTP logic.
//!
//! This is the pure data layer of the system.
//!
//! TODO:
//! - Add additional domain entities if future versions require them.
//!
//! End of File Notes:
//! Keep this crate dependency-light and stable.

use std::str::FromStr;
use std::fmt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;

/// Represents a discussion board.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: OffsetDateTime,
}

/// Represents a discussion thread inside a board.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thread {
    pub id: Uuid,
    pub board_id: Uuid,
    pub title: String,
    pub created_at: OffsetDateTime,
}

/// Represents a post inside a thread.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub thread_id: Uuid,
    pub content: String,
    pub created_at: OffsetDateTime,
}


impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::User => write!(f, "user"),
            Role::Admin => write!(f, "admin"),
        }
    }
}

impl FromStr for Role {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user" => Ok(Role::User),
            "admin" => Ok(Role::Admin),
            _ => Err(()),
        }
    }
}

/// User roles for access control.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
    User,
    Admin,
}

/// Represents an application user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub role: Role,
}

/// Represents a login session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub token: Uuid,
    pub user_id: Uuid,
    pub created_at: OffsetDateTime,
}



/// TESTS:
/// 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_equality_works() {
        assert_eq!(Role::User, Role::User);
        assert_ne!(Role::User, Role::Admin);
    }

    #[test]
    fn board_struct_can_be_created() {
        let board = Board {
            id: Uuid::new_v4(),
            name: "test".into(),
            description: "desc".into(),
            created_at: OffsetDateTime::now_utc(),
        };

        assert_eq!(board.name, "test");
    }
}