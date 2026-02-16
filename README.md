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

### 使用配置文件（推荐）

服务器支持通过配置文件自定义行为：

```bash
# 复制示例配置
cp config.toml.example config.toml

# 编辑配置
vim config.toml

# 启动服务器
./rust-webserver-linux-x86_64
```

**支持的配置文件路径：**
- `./config.toml`
- `./rust-webserver.toml`
- `~/.config/rust-webserver/config.toml`
- `/etc/rust-webserver/config.toml`

详细配置说明请查看 [CONFIG.md](CONFIG.md)

### 使用环境变量

```bash
# 自定义端口
SERVER_PORT=8080 ./rust-webserver-linux-x86_64

# 设置日志级别
LOG_LEVEL=debug ./rust-webserver-linux-x86_64
```

### 从源码运行

```bash
cargo run
```

服务器将在 `http://localhost:3000` 启动。

### 下载预编译二进制文件

我们提供以下平台的预编译二进制文件：

- **Linux x86_64**: `rust-webserver-linux-x86_64`
- **Linux ARM64**: `rust-webserver-linux-aarch64`
- **Windows x86_64**: `rust-webserver-windows-x86_64.exe`
- **macOS x86_64** (Intel): `rust-webserver-macos-x86_64`
- **macOS ARM64** (Apple Silicon): `rust-webserver-macos-aarch64`

**下载方式：**

1. 访问 [Releases 页面](https://github.com/cfanbo/openclaw-test/releases)
2. 下载适合你系统的二进制文件
3. 添加执行权限（Linux/macOS）：
   ```bash
   chmod +x rust-webserver-*
   ```
4. 运行：
   ```bash
   ./rust-webserver-linux-x86_64
   # 或 Windows
   rust-webserver-windows-x86_64.exe
   ```

### Docker 部署

使用 Docker 快速部署：

```bash
# 构建镜像
docker build -t rust-webserver:latest .

# 运行容器
docker run -d -p 3000:3000 --name rust-webserver rust-webserver:latest

# 使用环境变量配置
docker run -d -p 3000:3000 \
  -e SERVER_PORT=8080 \
  -e LOG_LEVEL=debug \
  --name rust-webserver \
  rust-webserver:latest
```

**详细 Docker 文档：** [DOCKER.md](DOCKER.md)

## 文档

| 文档 | 说明 |
|------|------|
| [README.md](README.md) | 项目概述和快速开始 |
| [CONFIG.md](CONFIG.md) | 配置文件详细说明 |
| [STRUCTURE.md](STRUCTURE.md) | 项目架构和模块说明 |
| [USAGE.md](USAGE.md) | 使用指南（systemd、手动部署） |
| [DOCKER.md](DOCKER.md) | Docker 部署完整指南 |

## CI/CD

项目使用 GitHub Actions 自动构建和发布：

- ✅ 自动构建多个平台的可执行文件
- ✅ 每次 push 到 master 分支触发构建
- ✅ 创建版本 tag 时自动发布 Release
- ✅ 上传编译好的二进制文件供下载

**触发发布：**

```bash
# 创建并推送版本 tag
git tag v1.0.0
git push origin v1.0.0
```

GitHub Actions 会自动：
1. 构建 5 个平台的可执行文件
2. 创建 GitHub Release
3. 上传所有二进制文件到 Release

## 技术栈

- **Axum** - Web 框架
- **Tokio** - 异步运行时
- **Serde** - 序列化/反序列化
- **GitHub Actions** - CI/CD 自动化
