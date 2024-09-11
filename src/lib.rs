mod db;
mod model;
mod router;

use crate::router::auth::get_users;
use axum::{routing::post, Router};

pub async fn run() {
    let app = Router::new().route("/users", post(get_users));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    // 나머지 서버 설정
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
