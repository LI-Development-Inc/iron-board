//! API Crate
//!
//! Developer Notes:
//! - This is the HTTP transport layer.
//! - It defines routes and handlers.
//! - It must NOT contain business rules.
//! - It must delegate to services.
//!
//! End of File Notes:
//! Keep this crate focused on request/response handling.

pub mod routes;
pub mod templates;