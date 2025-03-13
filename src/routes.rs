use axum::{extract::State, routing::{get, post}, Json, Router};
use sqlx::PgPool;
use crate::models::{User, NewUser};
use crate::state::AppState;
use crate::error::AppError;
use axum::http::StatusCode;
use axum::extract::rejection::JsonRejection;
use tracing::{info, error}; // Logging macros

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", get(get_users).post(create_user))
        .with_state(state)
}

async fn root() -> &'static str {
    "Hello, API!"
}

// Fetch all users from the database
async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<User>>, AppError> {
    info!("📡 Received GET /users request");

    let users = sqlx::query_as!(User, "SELECT id, name FROM users")
        .fetch_all(&state.db)
        .await
        .map_err(|err| {
            error!("❌ Database error in GET /users: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch users")
        })?;

    info!("✅ Successfully retrieved {} users", users.len());
    Ok(Json(users))
}

// Create a new user and insert into the database
async fn create_user(
    State(state): State<AppState>,
    result: Result<Json<NewUser>, JsonRejection>,
) -> Result<Json<User>, AppError> {
    info!("📡 Received POST /users request");

    // Handle cases where the request body is invalid or missing required fields
    let new_user = result.map_err(|_| {
        error!("❌ Invalid JSON body in POST /users");
        AppError::new(StatusCode::BAD_REQUEST, "Invalid request body: 'name' field is required")
    })?;

    // Insert the new user into the database and return the created user
    let inserted_user = sqlx::query_as!(
        User,
        "INSERT INTO users (name) VALUES ($1) RETURNING id, name",
        new_user.name
    )
    .fetch_one(&state.db)
    .await
    .map_err(|err| {
        error!("❌ Database error in POST /users: {:?}", err);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to insert user")
    })?;

    info!("✅ Successfully created user: {}", inserted_user.name);
    Ok(Json(inserted_user))
}
