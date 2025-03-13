use axum::{extract::State, routing::{get, post}, Json, Router};
use sqlx::PgPool;
use crate::models::{User, NewUser};
use crate::state::AppState;
use crate::error::AppError;
use axum::http::StatusCode;
use axum::extract::rejection::JsonRejection;
use tracing::{info, error}; // Logging macros
use crate::jwt::{create_jwt, verify_jwt};
use axum::extract::TypedHeader;
use axum::response::IntoResponse; // ✅ Fixes `IntoResponse` error
use headers::{Authorization, authorization::Bearer};

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/login", post(login)) // ✅ New login route
        .route("/users", get(get_users).post(create_user))
        .with_state(state)
}

async fn root() -> &'static str {
    "Hello, API!"
}

async fn login(Json(payload): Json<NewUser>) -> Result<Json<String>, AppError> {
    let token = create_jwt(&payload.name).map_err(|_| {
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to generate token")
    })?;

    Ok(Json(token))
}


// Fetch all users from the database
async fn get_users(
    (TypedHeader(auth), State(state)): (TypedHeader<Authorization<Bearer>>, State<AppState>),
) -> Result<Json<Vec<User>>, AppError> {
    let token = auth.token();

    let claims = verify_jwt(token).map_err(|_| {
        AppError::new(StatusCode::UNAUTHORIZED, "Invalid or expired token")
    })?;

    info!("✅ Authenticated user: {}", claims.sub);

    let users = sqlx::query_as!(User, "SELECT id, name FROM users")
        .fetch_all(&state.db)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch users"))?;

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
