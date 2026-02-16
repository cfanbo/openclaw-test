# 使用指南

## 快速开始

### 1. 下载预编译版本

从 [Releases](https://github.com/cfanbo/openclaw-test/releases) 下载适合你系统的版本。

### 2. 运行服务器

**Linux / macOS:**
```bash
# 下载并添加执行权限
chmod +x rust-webserver-linux-x86_64

# 运行
./rust-webserver-linux-x86_64
```

**Windows:**
```cmd
rust-webserver-windows-x86_64.exe
```

服务器将在 `http://localhost:3000` 启动。

### 3. 测试端点

```bash
# 检查版本
curl http://localhost:3000/version

# 心跳测试
curl http://localhost:3000/ping

# 健康检查
curl http://localhost:3000/health
```

## 自定义端口

默认端口是 `3000`。如需修改，请编辑 `src/main.rs` 中的端口号并重新编译。

## 系统服务配置

### Linux (systemd)

创建服务文件 `/etc/systemd/system/rust-webserver.service`:

```ini
[Unit]
Description=Rust WebServer
After=network.target

[Service]
Type=simple
User=nobody
WorkingDirectory=/opt/rust-webserver
ExecStart=/opt/rust-webserver/rust-webserver-linux-x86_64
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

启用并启动服务：
```bash
sudo systemctl enable rust-webserver
sudo systemctl start rust-webserver
```

### Docker (可选)

创建 `Dockerfile`:

```dockerfile
FROM rust:1.83 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/rust-webserver /usr/local/bin/
EXPOSE 3000
CMD ["rust-webserver"]
```

构建并运行：
```bash
docker build -t rust-webserver .
docker run -p 3000:3000 rust-webserver
```

## 故障排查

### 端口被占用
如果 3000 端口被占用，请先终止占用的进程：
```bash
# Linux/macOS
lsof -ti:3000 | xargs kill -9

# Windows
netstat -ano | findstr :3000
taskkill /PID <PID> /F
```

### 防火墙设置
确保防火墙允许 3000 端口：
```bash
# Linux (ufw)
sudo ufw allow 3000

# Linux (firewalld)
sudo firewall-cmd --add-port=3000/tcp --permanent
sudo firewall-cmd --reload
```

## 性能优化

- 使用 `--release` 模式编译以获得最佳性能
- 在生产环境建议使用反向代理（如 Nginx）
- 考虑启用 HTTPS（使用 Let's Encrypt + Nginx）

## 监控和日志

默认情况下，日志输出到标准输出。可以重定向到文件：
```bash
./rust-webserver-linux-x86_64 > server.log 2>&1 &
```

使用健康检查端点监控服务状态：
```bash
watch -n 5 'curl -s http://localhost:3000/health | jq .'
```
