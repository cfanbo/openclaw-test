# 配置指南

## 配置方式

服务器支持三种配置方式，优先级从高到低：

1. **环境变量** - 最高优先级
2. **配置文件** - 中等优先级
3. **默认值** - 最低优先级

---

## 配置文件

### 支持的配置文件路径（按优先级）

1. `./config.toml` - 当前目录的 config.toml
2. `./rust-webserver.toml` - 当前目录的 rust-webserver.toml
3. `~/.config/rust-webserver/config.toml` - 用户配置目录
4. `/etc/rust-webserver/config.toml` - 系统配置目录

服务器会按顺序查找，找到第一个配置文件即停止。

### 配置文件格式（TOML）

```toml
[server]
# 服务器监听地址
host = "0.0.0.0"

# 服务器监听端口
port = 3000

[logging]
# 日志级别：trace, debug, info, warn, error
level = "info"

# 日志格式：pretty, json, compact
format = "pretty"

# 是否显示日志颜色
color = true
```

---

## 配置项说明

### Server 配置

| 配置项 | 类型 | 默认值 | 说明 |
|--------|------|--------|------|
| `host` | String | 0.0.0.0 | 监听地址 |
| `port` | Integer | 3000 | 监听端口 |

### Logging 配置

| 配置项 | 类型 | 默认值 | 说明 |
|--------|------|--------|------|
| `level` | String | info | 日志级别（trace/debug/info/warn/error） |
| `format` | String | pretty | 日志格式（pretty/json/compact） |
| `color` | Boolean | true | 是否显示日志颜色 |

---

## 环境变量

支持的环境变量（会覆盖配置文件）：

| 环境变量 | 对应配置项 | 示例 |
|----------|-----------|------|
| `SERVER_HOST` | server.host | `127.0.0.1` |
| `SERVER_PORT` | server.port | `8080` |
| `LOG_LEVEL` | logging.level | `debug` |
| `LOG_FORMAT` | logging.format | `json` |

---

## 使用示例

### 1. 使用默认配置

```bash
./rust-webserver-linux-x86_64
```

默认配置：
- 地址：0.0.0.0:3000
- 日志级别：info
- 日志格式：pretty

### 2. 使用配置文件

```bash
# 复制示例配置
cp config.toml.example config.toml

# 编辑配置（可选）
vim config.toml

# 启动服务器
./rust-webserver-linux-x86_64
```

### 3. 使用环境变量

```bash
# 设置监听端口
SERVER_PORT=8080 ./rust-webserver-linux-x86_64

# 设置日志级别
LOG_LEVEL=debug ./rust-webserver-linux-x86_64

# 组合使用
SERVER_PORT=8080 LOG_LEVEL=debug LOG_FORMAT=json ./rust-webserver-linux-x86_64
```

### 4. 配置文件 + 环境变量

环境变量会覆盖配置文件中的值：

```bash
# 配置文件设置 port=3000
# 环境变量 SERVER_PORT=8080
# 实际使用端口 8080

SERVER_PORT=8080 ./rust-webserver-linux-x86_64
```

---

## 配置文件示例

### 开发环境

保存为 `config.development.toml`：

```toml
[server]
host = "127.0.0.1"
port = 3000

[logging]
level = "debug"
format = "pretty"
color = true
```

使用：
```bash
cp config.development.toml config.toml
./rust-webserver-linux-x86_64
```

### 生产环境

保存为 `config.production.toml`：

```toml
[server]
host = "0.0.0.0"
port = 8080

[logging]
level = "info"
format = "json"
color = false
```

使用：
```bash
cp config.production.toml config.toml
./rust-webserver-linux-x86_64
```

### Docker 环境

```dockerfile
FROM rust:1.83 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/rust-webserver /usr/local/bin/
COPY config.production.toml /etc/rust-webserver/config.toml
EXPOSE 8080
CMD ["rust-webserver"]
```

---

## 日志格式

### Pretty 格式（开发）

```bash
LOG_FORMAT=pretty ./rust-webserver-linux-x86_64
```

输出：
```
2024-02-16T08:00:00.000Z INFO rust-webserver: 🚀 Starting rust-webserver v1.1.0
2024-02-16T08:00:00.000Z INFO rust-webserver: 📡 Server listening on http://0.0.0.0:3000
```

### JSON 格式（生产）

```bash
LOG_FORMAT=json ./rust-webserver-linux-x86_64
```

输出：
```json
{"timestamp":"2024-02-16T08:00:00.000Z","level":"INFO","target":"rust-webserver","message":"🚀 Starting rust-webserver v1.1.0"}
```

### Compact 格式

```bash
LOG_FORMAT=compact ./rust-webserver-linux-x86_64
```

输出：
```
INFO rust-webserver: 🚀 Starting rust-webserver v1.1.0
```

---

## systemd 配置

创建 `/etc/systemd/system/rust-webserver.service`：

```ini
[Unit]
Description=Rust WebServer
After=network.target

[Service]
Type=simple
User=nobody
WorkingDirectory=/opt/rust-webserver
ExecStart=/opt/rust-webserver/rust-webserver-linux-x86_64
Environment="SERVER_PORT=8080"
Environment="LOG_LEVEL=info"
Environment="LOG_FORMAT=json"
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

启动服务：
```bash
sudo systemctl daemon-reload
sudo systemctl enable rust-webserver
sudo systemctl start rust-webserver
```

---

## 故障排查

### 配置文件未生效

1. 确认配置文件路径正确
2. 确认文件名是 `config.toml` 或 `rust-webserver.toml`
3. 查看启动日志中的配置加载信息

### 端口被占用

```bash
# 查看占用端口的进程
lsof -ti:3000 | xargs kill -9

# 或使用其他端口
SERVER_PORT=8080 ./rust-webserver-linux-x86_64
```

### 日志级别不生效

检查环境变量优先级高于配置文件：
```bash
# 查看当前环境变量
echo $LOG_LEVEL

# 临时取消环境变量
unset LOG_LEVEL
./rust-webserver-linux-x86_64
```
