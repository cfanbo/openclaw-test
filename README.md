# Rust WebServer

一个简单的 Rust web server，包含三个 API 接口。

## API 接口

### GET `/version`
返回当前版本号

```bash
curl http://localhost:3000/version
```

响应：
```json
{
  "version": "0.1.0",
  "name": "rust-webserver"
}
```

### GET `/ping`
返回 `pong`，用于简单的连通性测试

```bash
curl http://localhost:3000/ping
```

响应：
```
pong
```

### GET `/health`
返回服务健康状态

```bash
curl http://localhost:3000/health
```

响应：
```json
{
  "status": "ok",
  "version": "0.1.0",
  "service": "rust-webserver",
  "timestamp": 1771228533
}
```

## 运行

```bash
cargo run
```

服务器将在 `http://localhost:3000` 启动。

## 技术栈

- **Axum** - Web 框架
- **Tokio** - 异步运行时
