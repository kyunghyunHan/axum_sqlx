use crate::db::connection::connection;
use crate::model::auth::{LoginUser, User};
use axum::{response::Json as JsonResponse, Json};
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

pub async fn login(input: Json<LoginUser>) -> JsonResponse<Value> {
    let connection: sqlx::Pool<sqlx::Postgres> = connection().await;
    let query =
        sqlx::query("SELECT id, user_id, user_pw FROM users WHERE user_id = $1 AND user_pw = $2")
            .bind(&input.user_id) // user_id 값 바인딩
            .bind(&input.user_pw) // user_pw 값 바인딩
            .fetch_all(&connection)
            .await
            .expect("Failed to fetch users.");
    let result = !query.is_empty();
    println!("{:?}", result);

    // let users: Vec<User> = query
    //     .into_iter()
    //     .map(|row| User {
    //         id: row.get("id"),
    //         user_id: row.get("user_id"),
    //         user_pw: row.get("user_pw"),
    //     })
    //     .collect();
    JsonResponse(json!({ "result": result}))
}
