use axum::{
    routing::get,
    Json, Router,
};
use serde_json::Value;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // 构建路由
    let app = Router::new()
        .route("/version", get(version))
        .route("/ping", get(ping));

    // 绑定地址
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Server listening on http://{}", addr);

    // 启动服务器
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}

// /version - 返回版本号
async fn version() -> Json<Value> {
    Json(serde_json::json!({
        "version": env!("CARGO_PKG_VERSION"),
        "name": env!("CARGO_PKG_NAME")
    }))
}

// /ping - 返回 pong
async fn ping() -> String {
    "pong".to_string()
}
