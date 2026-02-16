use axum::{Router, routing::get};
use crate::handlers;

/// 创建应用路由
/// 
/// 配置所有的 API 端点
pub fn create_router() -> Router {
    Router::new()
        .route("/version", get(handlers::version))
        .route("/ping", get(handlers::ping))
        .route("/health", get(handlers::health))
}
