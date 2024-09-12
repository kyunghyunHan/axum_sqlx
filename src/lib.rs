mod db;
mod model;
mod router;

use crate::router::{
    auth::{get_users, join, login, secession, update_user},
    test::test,
};
use axum::{routing::post, Router};

pub async fn run() {
    let app = Router::new()
        .route("/auth/users", post(get_users))
        .route("/auth/join", post(join))
        .route("/auth/login", post(login))
        .route("/auth/secession", post(secession))
        .route("/auth/update", post(update_user))
        .route("/test/test", post(test));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    // 나머지 서버 설정
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
