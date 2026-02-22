//! Authentication Crate
//!
//! Developer Notes:
//! - Provides password hashing and verification.
//! - Generates secure session tokens.
//! - Contains NO database logic.
//! - Contains NO HTTP logic.
//!
//! This crate provides cryptographic primitives for Lean v1.
//!
//! TODO:
//! - Future versions may include additional token utilities.
//!
//! End of File Notes:
//! Keep this crate focused and security-oriented.

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;
use uuid::Uuid;
use thiserror::Error;

use models::Role;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Password hashing failed")]
    HashingFailed,

    #[error("Password verification failed")]
    VerificationFailed,
}

/// Hash a password using Argon2.
pub fn hash_password(password: &str) -> Result<String, AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AuthError::HashingFailed)?
        .to_string();

    Ok(hash)
}

/// Verify a password against a stored hash.
pub fn verify_password(
    stored_hash: &str,
    password: &str,
) -> Result<bool, AuthError> {
    let parsed_hash =
        PasswordHash::new(stored_hash)
            .map_err(|_| AuthError::VerificationFailed)?;

    let argon2 = Argon2::default();

    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

/// Generate a new secure session token.
pub fn generate_session_token() -> Uuid {
    Uuid::new_v4()
}

/// Check if role is admin.
pub fn is_admin(role: &Role) -> bool {
    matches!(role, Role::Admin)
}





//// TESTS:
/// 
/// 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_hash_and_verify() {
        let password = "secure_password";

        let hash = hash_password(password).unwrap();
        let valid = verify_password(&hash, password).unwrap();

        assert!(valid);
    }

    #[test]
    fn session_token_is_unique() {
        let t1 = generate_session_token();
        let t2 = generate_session_token();
        assert_ne!(t1, t2);
    }
}