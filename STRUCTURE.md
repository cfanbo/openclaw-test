# 项目结构

本项目按照 Rust 社区标准实践进行组织。

## 目录结构

```
rust-webserver/
├── .github/
│   └── workflows/
│       └── build.yml          # CI/CD 配置
├── src/
│   ├── main.rs                # 应用入口点
│   ├── config/                # 配置模块
│   │   └── mod.rs             # 服务器配置
│   ├── handlers/              # 请求处理器
│   │   ├── mod.rs             # 处理器模块声明
│   │   ├── health.rs          # 健康检查处理器
│   │   ├── ping.rs            # Ping 处理器
│   │   └── version.rs         # 版本信息处理器
│   ├── models/                # 数据模型
│   │   └── mod.rs             # 响应数据结构
│   └── routes/                # 路由配置
│       └── mod.rs             # 路由定义
├── tests/                     # 集成测试（可选）
├── Cargo.toml                 # 项目配置和依赖
├── Cargo.lock                 # 依赖版本锁定
├── README.md                  # 项目说明
├── USAGE.md                   # 使用指南
└── STRUCTURE.md               # 本文件
```

## 模块说明

### `src/main.rs`
应用程序的入口点，负责：
- 初始化日志系统
- 加载配置
- 启动 HTTP 服务器
- 集成测试

### `src/config/`
服务器配置相关：
- 支持硬编码默认值
- 支持环境变量配置
- 类型安全的配置访问

**环境变量：**
- `SERVER_HOST`: 服务器地址（默认：0.0.0.0）
- `SERVER_PORT`: 服务器端口（默认：3000）

### `src/handlers/`
HTTP 请求处理器，每个文件处理一个端点：
- `health.rs`: GET /health - 健康检查
- `ping.rs`: GET /ping - 连通性测试
- `version.rs`: GET /version - 版本信息

### `src/models/`
数据模型定义：
- 请求/响应的数据结构
- 使用 serde 进行序列化
- 提供构造函数和方法

### `src/routes/`
路由配置：
- 定义所有 API 端点
- 将路径映射到处理器
- 应用中间件

## 设计原则

### 1. 模块化
每个模块都有明确的职责：
- handlers 只处理 HTTP 请求
- models 只定义数据结构
- routes 只定义路由映射

### 2. 可测试性
- 处理器是独立的函数，易于单元测试
- 集成测试位于 main.rs 的 tests 模块

### 3. 可扩展性
添加新端点的步骤：
1. 在 `models/mod.rs` 添加数据模型
2. 在 `handlers/` 创建新的处理器文件
3. 在 `handlers/mod.rs` 导出处理器
4. 在 `routes/mod.rs` 注册路由

### 4. 配置管理
- 支持环境变量
- 提供合理的默认值
- 类型安全的配置访问

## 添加新端点示例

假设要添加 `/users` 端点：

**1. 创建数据模型**
```rust
// src/models/user.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
}
```

**2. 创建处理器**
```rust
// src/handlers/user.rs
use axum::Json;
use crate::models::User;

pub async fn get_users() -> Json<Vec<User>> {
    Json(vec![])
}
```

**3. 注册路由**
```rust
// src/routes/mod.rs
use crate::handlers;

pub fn create_router() -> Router {
    Router::new()
        .route("/users", get(handlers::get_users))
        // ... 其他路由
}
```

## 测试

运行所有测试：
```bash
cargo test
```

运行特定测试：
```bash
cargo test test_health_endpoint
```

运行测试并显示输出：
```bash
cargo test -- --nocapture
```

## 依赖说明

| 依赖 | 版本 | 用途 |
|------|------|------|
| axum | 0.7 | Web 框架 |
| tokio | 1 | 异步运行时 |
| serde | 1 | 序列化/反序列化 |
| tower-http | 0.5 | HTTP 中间件（trace、timeout） |
| tracing | 0.1 | 结构化日志 |
| anyhow | 1 | 错误处理 |

## 最佳实践

### ✅ 推荐做法
- 每个处理器只做一件事
- 使用类型安全的模型
- 为公共函数添加文档注释
- 编写单元测试和集成测试
- 使用环境变量管理配置

### ❌ 避免做法
- 在处理器中直接访问数据库（应该使用服务层）
- 硬编码配置值
- 忽略错误处理
- 过度使用 unwrap()

## 参考资源

- [Axum 官方文档](https://docs.rs/axum/)
- [Rust 项目布局](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html)
- [Tower 中间件](https://docs.rs/tower/)
