use axum::{extract::State, routing::{get, post}, Json, Router};
use sqlx::PgPool;
use crate::models::{User, NewUser};
use crate::state::AppState;
use crate::error::AppError; // ✅ Import custom error type
use axum::http::StatusCode;
use axum::extract::rejection::JsonRejection; // ✅ Import JSON parsing error handling

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", get(get_users).post(create_user))
        .with_state(state)
}

async fn root() -> &'static str {
    "Hello, API!"
}

// ✅ Updated: Proper error handling
async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<User>>, AppError> {
    let users = sqlx::query_as!(User, "SELECT id, name FROM users")
        .fetch_all(&state.db)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch users"))?;

    Ok(Json(users))
}

// ✅ Updated: Handle JSON parsing errors for `create_user`
async fn create_user(
    State(state): State<AppState>,
    result: Result<Json<NewUser>, JsonRejection>, // ✅ Handle JSON errors
) -> Result<Json<User>, AppError> {
    let new_user = result.map_err(|_| {
        AppError::new(StatusCode::BAD_REQUEST, "Invalid request body: 'name' field is required")
    })?;

    let inserted_user = sqlx::query_as!(
        User,
        "INSERT INTO users (name) VALUES ($1) RETURNING id, name",
        new_user.name
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to insert user"))?;

    Ok(Json(inserted_user))
}
