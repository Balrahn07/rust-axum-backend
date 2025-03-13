mod models;
mod routes;
mod state;

use axum::Server;
use dotenvy::dotenv;
use routes::create_routes;
use sqlx::PgPool;
use state::AppState;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok(); // ✅ This ensures the .env file is loaded

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("✅ Using DATABASE_URL: {}", database_url);

    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("✅ Successfully connected to database");

    let app_state = AppState { db: db_pool.clone() };
    let app = create_routes(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server running at http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
