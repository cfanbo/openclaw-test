use axum::Json;
use crate::models::VersionResponse;

/// 版本信息处理器
/// 
/// 返回当前应用的名称和版本号
pub async fn version() -> Json<VersionResponse> {
    Json(VersionResponse::new())
}
