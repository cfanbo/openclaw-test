# 多阶段构建，优化镜像大小

# 构建阶段
FROM rust:1.83 AS builder

WORKDIR /app

# 复制 Cargo 配置
COPY Cargo.toml Cargo.lock ./

# 创建虚拟源码目录以利用 Docker 缓存
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 复制真实源码
COPY src ./src

# 构建应用
RUN touch src/main.rs && \
    cargo build --release

# 运行阶段
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/rust-webserver /usr/local/bin/rust-webserver

# 创建配置目录
RUN mkdir -p /etc/rust-webserver

# 复制生产环境配置（可选）
COPY config.production.toml /etc/rust-webserver/config.toml

# 暴露端口
EXPOSE 3000

# 健康检查
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# 设置用户
RUN useradd -m -u 1000 rustweb && \
    chown -R rustweb:rustweb /etc/rust-webserver

USER rustweb

# 启动应用
CMD ["rust-webserver"]
