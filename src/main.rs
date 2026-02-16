mod config;
mod handlers;
mod models;
mod routes;

use axum::Router;
use config::Config;
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 加载配置（配置文件 + 环境变量 + 默认值）
    let config = Config::load();

    // 初始化日志系统
    config.logging.init()?;

    let addr = config.server.addr()?;

    tracing::info!("🚀 Starting {} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    tracing::info!("📡 Server listening on http://{}", addr);
    tracing::info!("📊 Log level: {}", config.logging.level);
    tracing::info!("📝 Log format: {}", config.logging.format);

    // 创建路由
    let app = create_app();

    // 启动服务器
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .await
        .map_err(|e| anyhow::anyhow!("Server error: {}", e))?;

    Ok(())
}

/// 创建应用实例
fn create_app() -> Router {
    routes::create_router()
        .layer(TraceLayer::new_for_http())
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_endpoint() {
        let app = create_app();
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/health")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_ping_endpoint() {
        let app = create_app();
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/ping")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_version_endpoint() {
        let app = create_app();
        let response = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/version")
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
