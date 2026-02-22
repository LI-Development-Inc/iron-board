//! Configuration Crate
//!
//! Developer Notes:
//! - Defines application configuration structures.
//! - Loads environment variables.
//! - Provides sensible defaults.
//! - Contains NO business logic.
//! - Contains NO database logic.
//!
//! This crate centralizes runtime configuration.
//!
//! TODO:
//! - Optionally support config file loading in future versions.
//!
//! End of File Notes:
//! Keep configuration simple and explicit.

use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub database_path: String,
    pub server_address: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            database_path: "rusty_board.db".into(),
            server_address: "0.0.0.0:3000".into(),
        }
    }
}

impl AppConfig {
    /// Load configuration from environment variables.
    ///
    /// Environment Variables:
    /// - DATABASE_PATH
    /// - SERVER_ADDRESS
    pub fn from_env() -> Self {
        Self {
            database_path: env::var("DATABASE_PATH")
                .unwrap_or_else(|_| "rusty_board.db".into()),
            server_address: env::var("SERVER_ADDRESS")
                .unwrap_or_else(|_| "0.0.0.0:3000".into()),
        }
    }
}



/// TESTS
/// 
/// 
/// 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_valid() {
        let config = AppConfig::default();
        assert!(!config.database_path.is_empty());
        assert!(!config.server_address.is_empty());
    }
}