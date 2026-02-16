# Docker 部署指南

## 快速开始

### 1. 使用预构建镜像（如果已发布）

```bash
docker pull cfanbo/openclaw-test:latest
docker run -p 3000:3000 cfanbo/openclaw-test:latest
```

### 2. 从源码构建镜像

```bash
# 克隆仓库
git clone https://github.com/cfanbo/openclaw-test.git
cd openclaw-test

# 构建镜像
docker build -t rust-webserver:latest .

# 运行容器
docker run -p 3000:3000 rust-webserver:latest
```

---

## Docker 使用示例

### 基本运行

```bash
docker run -d \
  --name rust-webserver \
  -p 3000:3000 \
  rust-webserver:latest
```

### 自定义端口

```bash
docker run -d \
  --name rust-webserver \
  -p 8080:3000 \
  rust-webserver:latest
```

### 使用环境变量配置

```bash
docker run -d \
  --name rust-webserver \
  -p 3000:3000 \
  -e SERVER_PORT=8080 \
  -e LOG_LEVEL=debug \
  rust-webserver:latest
```

### 挂载配置文件

```bash
# 创建自定义配置
cat > my-config.toml << EOF
[server]
host = "0.0.0.0"
port = 3000

[logging]
level = "info"
format = "json"
color = false
EOF

# 运行并挂载配置文件
docker run -d \
  --name rust-webserver \
  -p 3000:3000 \
  -v $(pwd)/my-config.toml:/etc/rust-webserver/config.toml:ro \
  rust-webserver:latest
```

### 持久化日志

```bash
docker run -d \
  --name rust-webserver \
  -p 3000:3000 \
  -v $(pwd)/logs:/var/log \
  rust-webserver:latest
```

---

## Docker Compose

创建 `docker-compose.yml` 文件：

```yaml
version: '3.8'

services:
  webserver:
    build: .
    image: rust-webserver:latest
    container_name: rust-webserver
    ports:
      - "3000:3000"
    environment:
      - SERVER_PORT=3000
      - LOG_LEVEL=info
      - LOG_FORMAT=json
    volumes:
      # 挂载配置文件（可选）
      - ./config.toml:/etc/rust-webserver/config.toml:ro
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3000/health"]
      interval: 30s
      timeout: 3s
      retries: 3
      start_period: 5s
```

使用 Docker Compose：

```bash
# 启动服务
docker-compose up -d

# 查看日志
docker-compose logs -f

# 停止服务
docker-compose down

# 重启服务
docker-compose restart
```

---

## 生产环境部署

### 1. 使用 Nginx 反向代理

**Nginx 配置** (`nginx.conf`):

```nginx
events {
    worker_connections 1024;
}

http {
    upstream rust_webserver {
        server rust-webserver:3000;
    }

    server {
        listen 80;
        server_name example.com;

        location / {
            proxy_pass http://rust_webserver;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header X-Forwarded-Proto $scheme;
        }
    }
}
```

**Docker Compose 配置**:

```yaml
version: '3.8'

services:
  webserver:
    image: rust-webserver:latest
    restart: unless-stopped
    environment:
      - LOG_LEVEL=info
      - LOG_FORMAT=json
    networks:
      - app-network

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
      - ./ssl:/etc/nginx/ssl:ro
    depends_on:
      - webserver
    restart: unless-stopped
    networks:
      - app-network

networks:
  app-network:
    driver: bridge
```

### 2. 健康检查

容器内置健康检查，每 30 秒检查一次：

```bash
# 查看健康状态
docker inspect --format='{{.State.Health.Status}}' rust-webserver

# 查看健康检查日志
docker inspect --format='{{range .State.Health.Log}}{{.Output}}{{end}}' rust-webserver
```

### 3. 资源限制

```yaml
version: '3.8'

services:
  webserver:
    image: rust-webserver:latest
    deploy:
      resources:
        limits:
          cpus: '0.5'
          memory: 512M
        reservations:
          cpus: '0.25'
          memory: 256M
```

---

## 多阶段构建说明

项目使用多阶段构建优化镜像大小：

1. **构建阶段** - 使用 `rust:1.83` 镜像编译应用
2. **运行阶段** - 使用 `debian:bookworm-slim` 镜像运行应用

**优势：**
- ✅ 最终镜像大小约 80MB（vs 1.5GB+）
- ✅ 只包含运行时依赖
- ✅ 更安全（不包含编译工具）
- ✅ 更快的部署速度

---

## 镜像优化

### 查看镜像大小

```bash
docker images | grep rust-webserver
```

### 进一步优化（可选）

如果需要更小的镜像，可以使用 Alpine 基础镜像：

```dockerfile
# 运行阶段（使用 Alpine）
FROM alpine:latest

RUN apk add --no-cache ca-certificates

COPY --from=builder /app/target/release/rust-webserver /usr/local/bin/rust-webserver

EXPOSE 3000

CMD ["rust-webserver"]
```

---

## 故障排查

### 查看容器日志

```bash
docker logs rust-webserver
docker logs -f rust-webserver  # 实时查看
```

### 进入容器

```bash
docker exec -it rust-webserver sh
```

### 检查端口占用

```bash
docker port rust-webserver
```

### 重启容器

```bash
docker restart rust-webserver
```

### 调试配置

```bash
# 运行并进入容器检查配置
docker run -it --rm \
  -v $(pwd)/config.toml:/etc/rust-webserver/config.toml:ro \
  rust-webserver:latest sh
```

---

## 安全最佳实践

1. **使用非 root 用户**
   - Dockerfile 已配置 `USER rustweb`

2. **只读文件系统**
   ```yaml
   services:
     webserver:
       read_only: true
       tmpfs:
         - /tmp
   ```

3. **资源限制**
   ```yaml
   deploy:
     resources:
       limits:
         cpus: '1'
         memory: 1G
   ```

4. **网络隔离**
   ```yaml
   networks:
     app-network:
       internal: true
   ```

5. **定期更新基础镜像**
   ```bash
   docker pull rust:1.83
   docker build -t rust-webserver:latest .
   ```

---

## CI/CD 集成

### GitHub Actions 示例

```yaml
name: Docker Build and Push

on:
  push:
    branches: [master]
    tags:
      - 'v*'

jobs:
  docker:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Set up Docker Buildx
        uses: docker/setup-buildx-action@v3

      - name: Login to Docker Hub
        uses: docker/login-action@v3
        with:
          username: ${{ secrets.DOCKER_USERNAME }}
          password: ${{ secrets.DOCKER_PASSWORD }}

      - name: Build and push
        uses: docker/build-push-action@v5
        with:
          context: .
          push: true
          tags: |
            cfanbo/openclaw-test:latest
            cfanbo/openclaw-test:${{ github.ref_name }}
          cache-from: type=gha
          cache-to: type=gha,mode=max
```

---

## 性能监控

### 使用 Docker Stats

```bash
docker stats rust-webserver
```

### 日志收集

```yaml
services:
  webserver:
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
```

---

## 清理

### 删除容器和镜像

```bash
# 停止并删除容器
docker stop rust-webserver
docker rm rust-webserver

# 删除镜像
docker rmi rust-webserver:latest

# 清理未使用的资源
docker system prune -a
```

---

## 参考资料

- [Docker 官方文档](https://docs.docker.com/)
- [Docker Compose 文档](https://docs.docker.com/compose/)
- [Docker 最佳实践](https://docs.docker.com/develop/dev-best-practices/)
