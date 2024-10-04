mod db;
mod model;
mod router;
use axum::{
    response::{Html, Json as JsonResponse},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub user_id: String,
    // pub user_pw: String,
}

pub async fn index() -> Html<&'static str> {
    println!("{}", 1);
    let html_content = include_str!("../index.html");
    Html(html_content)
}

pub async fn index2(input: Json<LoginUser>) -> JsonResponse<Value> {
    println!("{:?}", input);
    JsonResponse(json!({ "result": 1}))
}
pub async fn run() {
    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        // .route("/auth/users", post(get_users))
        // .route("/auth/join", post(join))
        // .route("/auth/login", post(login))
        // .route("/auth/secession", post(secession))
        // .route("/auth/update", post(update_user))
        // .route("/test/test", post(test));
        .route("/ip", post(index2))
        .route("/", get(index))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    // 나머지 서버 설정
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
