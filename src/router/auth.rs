use std::result;

use crate::db::connection::connection;
use crate::model::auth::{JoinUser, LoginUser, User};
use axum::{response::Json as JsonResponse, Json};
use serde_json::{json, Value};
use sqlx::{query, Row};
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

pub async fn join(input: Json<JoinUser>) -> JsonResponse<Value> {
    let connection: sqlx::Pool<sqlx::Postgres> = connection().await;

    let query =
        sqlx::query("SELECT id, user_id, user_pw FROM users WHERE user_id = $1 AND user_pw = $2")
            .bind(&input.user_id) // user_id 값 바인딩
            .bind(&input.user_pw) // user_pw 값 바인딩
            .fetch_all(&connection)
            .await
            .expect("Failed to fetch users.");

    let result = query.is_empty();
    println!("{}", result);

    match result {
        true => {
            let result = sqlx::query("INSERT INTO users (user_id, user_pw) VALUES ($1, $2)")
                .bind(&input.user_id) // user_id 값 바인딩
                .bind(&input.user_pw) // user_pw 값 바인딩
                .execute(&connection) // 쿼리 실행
                .await
                .expect("Failed to insert user");

            match result {
                exec_result => {
                    if exec_result.rows_affected() > 0 {
                        JsonResponse(json!({ "result": true}))
                        // 추가 작업이 필요할 경우 여기에 작성
                    } else {
                        JsonResponse(json!({ "result":false,"msg":"You are already registered"}))
                    }
                }
            }
        }
        false => JsonResponse(json!({ "result":false,"msg":"You are already registered"})),
    }
}
