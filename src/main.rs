use axum::{routing::get, Json, Router};
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
}

async fn get_users(pool: PgPool) -> Json<Vec<User>> {
    let users = sqlx::query!(User, "SELECT id, name FROM users")
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch users.");

    Json(users)
}
#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://admin:qwer1234@localhost/test")
        .await
        .expect("Failed to create pool.");
    let app = Router::new().route("/users", get(get_users));

    // 나머지 서버 설정
}
