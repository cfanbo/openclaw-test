use axum::Json;
use crate::models::HealthResponse;

/// 健康检查处理器
/// 
/// 返回服务健康状态，包括状态、版本、服务名称和时间戳
pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse::new())
}
