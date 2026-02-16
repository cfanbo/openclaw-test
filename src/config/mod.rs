use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::Path;

/// 服务器完整配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// 服务器配置
    #[serde(default)]
    pub server: ServerConfig,
    /// 日志配置
    #[serde(default)]
    pub logging: LoggingConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

impl Config {
    /// 从配置文件加载
    ///
    /// 支持的配置文件路径（按优先级）：
    /// 1. ./config.toml
    /// 2. ./rust-webserver.toml
    /// 3. ~/.config/rust-webserver/config.toml
    /// 4. /etc/rust-webserver/config.toml
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    /// 自动查找并加载配置文件
    ///
    /// 按优先级查找配置文件，找到即停止
    pub fn auto_load() -> Self {
        // 尝试多个配置文件路径
        let config_paths = [
            "./config.toml",
            "./rust-webserver.toml",
            "~/.config/rust-webserver/config.toml",
            "/etc/rust-webserver/config.toml",
        ];

        for path in config_paths {
            // 展开波浪号
            let expanded = shellexpand::tilde(path);
            if std::path::Path::new(expanded.as_ref()).exists() {
                match Self::from_file(expanded.as_ref()) {
                    Ok(config) => {
                        tracing::info!("✅ 加载配置文件: {}", path);
                        return config;
                    }
                    Err(e) => {
                        tracing::warn!("⚠️  配置文件 {} 解析失败: {}", path, e);
                    }
                }
            }
        }

        tracing::info!("📝 使用默认配置");
        Self::default()
    }

    /// 从环境变量覆盖配置
    pub fn override_with_env(&mut self) {
        // 服务器配置
        if let Ok(host) = std::env::var("SERVER_HOST") {
            self.server.host = host;
        }
        if let Ok(port) = std::env::var("SERVER_PORT") {
            if let Ok(p) = port.parse::<u16>() {
                self.server.port = p;
            }
        }

        // 日志配置
        if let Ok(level) = std::env::var("LOG_LEVEL") {
            self.logging.level = level;
        }
        if let Ok(format) = std::env::var("LOG_FORMAT") {
            self.logging.format = format;
        }
    }

    /// 加载配置（自动查找配置文件 + 环境变量覆盖）
    pub fn load() -> Self {
        let mut config = Self::auto_load();
        config.override_with_env();
        config
    }
}

/// 服务器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// 监听地址（默认：0.0.0.0）
    #[serde(default = "default_host")]
    pub host: String,
    /// 监听端口（默认：3000）
    #[serde(default = "default_port")]
    pub port: u16,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    3000
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
        }
    }
}

impl ServerConfig {
    /// 获取 SocketAddr
    pub fn addr(&self) -> anyhow::Result<SocketAddr> {
        format!("{}:{}", self.host, self.port)
            .parse()
            .map_err(|e| anyhow::anyhow!("Invalid server address: {}", e))
    }
}

/// 日志配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// 日志级别：trace, debug, info, warn, error（默认：info）
    #[serde(default = "default_log_level")]
    pub level: String,
    /// 日志格式：pretty, json, compact（默认：pretty）
    #[serde(default = "default_log_format")]
    pub format: String,
    /// 是否显示日志颜色（默认：true）
    #[serde(default = "default_log_color")]
    pub color: bool,
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_log_format() -> String {
    "pretty".to_string()
}

fn default_log_color() -> bool {
    true
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            format: default_log_format(),
            color: default_log_color(),
        }
    }
}

impl LoggingConfig {
    /// 解析日志级别为 tracing::Level
    pub fn parse_level(&self) -> tracing::Level {
        self.level.to_lowercase().as_str().parse().unwrap_or(tracing::Level::INFO)
    }

    /// 初始化日志系统
    pub fn init(&self) -> anyhow::Result<()> {
        let level = self.parse_level();

        match self.format.as_str() {
            "json" => {
                tracing_subscriber::fmt()
                    .json()
                    .with_max_level(level)
                    .init();
            }
            "compact" => {
                tracing_subscriber::fmt()
                    .compact()
                    .with_max_level(level)
                    .init();
            }
            "pretty" | _ => {
                tracing_subscriber::fmt()
                    .pretty()
                    .with_max_level(level)
                    .init();
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.server.host, "0.0.0.0");
        assert_eq!(config.server.port, 3000);
        assert_eq!(config.logging.level, "info");
    }

    #[test]
    fn test_server_addr() {
        let config = ServerConfig::default();
        assert!(config.addr().is_ok());
    }

    #[test]
    fn test_log_level_parse() {
        let log_config = LoggingConfig {
            level: "debug".to_string(),
            ..Default::default()
        };
        assert_eq!(log_config.parse_level(), tracing::Level::DEBUG);
    }
}
