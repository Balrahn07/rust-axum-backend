mod models;
mod routes;
mod state;
mod error; // Include error handling module

use axum::Server;
use dotenvy::dotenv;
use routes::create_routes;
use sqlx::PgPool;
use state::AppState;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{info, error}; // Logging macros for structured output
use tracing_subscriber; // Logging system for Rust

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize structured logging
    tracing_subscriber::fmt::init();
    info!("🚀 Starting Rust Backend...");

    // Retrieve database URL from environment variables
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Attempt to connect to PostgreSQL
    let db_pool = match PgPool::connect(&database_url).await {
        Ok(pool) => {
            info!("✅ Successfully connected to PostgreSQL");
            pool
        }
        Err(err) => {
            error!("❌ Database connection failed: {:?}", err);
            panic!("Database connection failed");
        }
    };

    // Create application state containing the database connection pool
    let app_state = AppState { db: db_pool.clone() };
    let app = create_routes(app_state);

    // Define the address where the server will run
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("🚀 Server running at http://{}", addr);

    // Start the Axum HTTP server
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
