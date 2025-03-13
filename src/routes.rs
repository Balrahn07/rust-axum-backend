use axum::{extract::State, routing::{get, post}, Json, Router};
use sqlx::PgPool;
use crate::models::{User, NewUser}; // ✅ Import both `User` and `NewUser`
use crate::state::AppState;

pub fn create_routes(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", get(get_users).post(create_user)) // ✅ POST method now works correctly
        .with_state(state)
}

async fn root() -> &'static str {
    "Hello, API!"
}

// ✅ Fix: Use `NewUser` for POST requests
async fn create_user(State(state): State<AppState>, Json(new_user): Json<NewUser>) -> Json<User> {
    let inserted_user = sqlx::query_as!(
        User,
        "INSERT INTO users (name) VALUES ($1) RETURNING id, name",
        new_user.name  // ✅ Correctly using `NewUser`
    )
    .fetch_one(&state.db)
    .await
    .unwrap();

    Json(inserted_user) // ✅ Returns `User` (including `id`)
}

// ✅ No changes needed here
async fn get_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = sqlx::query_as!(User, "SELECT id, name FROM users")
        .fetch_all(&state.db)
        .await
        .unwrap();

    Json(users)
}
