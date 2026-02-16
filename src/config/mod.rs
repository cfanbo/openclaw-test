use std::net::SocketAddr;

/// 服务器配置
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 3000,
        }
    }
}

impl ServerConfig {
    /// 创建新的服务器配置
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
        }
    }

    /// 从环境变量加载配置
    /// 
    /// 支持的环境变量：
    /// - SERVER_HOST: 服务器地址（默认：0.0.0.0）
    /// - SERVER_PORT: 服务器端口（默认：3000）
    pub fn from_env() -> Self {
        let host = std::env::var("SERVER_HOST")
            .unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = std::env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000);

        Self { host, port }
    }

    /// 获取 SocketAddr
    pub fn addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port)
            .parse()
            .expect("Invalid server address")
    }
}
