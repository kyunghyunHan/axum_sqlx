use crate::db::connection::connection;
use crate::model::auth::User;
use axum::response::Json as JsonResponse;
use serde_json::{json, Value};
use sqlx::Row;
pub async fn get_users() -> JsonResponse<Value> {
    let connection = connection().await;
    let users = sqlx::query("SELECT id, user_id,user_pw FROM users")
        .fetch_all(&connection)
        .await
        .expect("Failed to fetch users.");

    let users: Vec<User> = users
        .into_iter()
        .map(|row| User {
            id: row.get("id"),
            user_id: row.get("user_id"),
            user_pw: row.get("user_pw"),
        })
        .collect();
    JsonResponse(json!({ "result": true,"users":users}))
}
