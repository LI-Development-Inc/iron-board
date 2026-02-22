//! Rusty-Board Lean v1
//!
//! Developer Notes:
//! - This is the composition root.
//! - It wires all crates together.
//! - It contains NO business logic.
//! - It only starts the application.
//!
//! TODO:
//! - Load configuration
//! - Initialize database
//! - Build services
//! - Start HTTP server
//!
//! End of File Notes:
//! Keep this file minimal and stable.

use std::sync::Arc;

use axum::Router;
use tower_http::services::ServeDir;

use storage::{create_connection, init_database};
use services::ServiceLayer;
use api::routes::{create_router, AppState};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("Starting Rusty-Board Lean v1...");

    // Initialize database
    let conn = create_connection("rusty_board.db")
        .expect("Failed to open database");

    init_database(&conn)
        .expect("Failed to initialize database");

    // Build application state
    let services = Arc::new(ServiceLayer);
    let db = Arc::new(conn);

    let state = AppState { services, db };

    // Create router
    let app = create_router(state)
        .nest_service("/static", ServeDir::new("static"));

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind port");

    tracing::info!("Server running on http://localhost:3000");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}