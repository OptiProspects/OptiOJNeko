# 使用官方 Alpine 镜像作为构建阶段
FROM alpine:latest as builder

# 设置环境变量
ENV TZ=Asia/Shanghai

RUN echo "http://mirrors.aliyun.com/alpine/v3.19/main" > /etc/apk/repositories && \
    echo "http://mirrors.aliyun.com/alpine/v3.19/community" >> /etc/apk/repositories

# 安装必要的工具和编译器
RUN apk update && apk add --no-cache \
    build-base \
    rust \
    cargo \
    protobuf-dev \
    protoc

# 配置 Rust 环境
ENV RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
ENV RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
ENV PATH=/root/.cargo/bin:$PATH

# 创建工作目录
WORKDIR /workspace/opti-neko

# 首先只复制 Cargo.toml 和 Cargo.lock 来缓存依赖
COPY Cargo.toml Cargo.lock ./

# 创建一个空的 main.rs 来构建依赖
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src/

# 复制实际的源代码
COPY . .

# 重新构建项目
RUN cargo build --release

# 使用新的镜像作为运行阶段
FROM alpine:latest

# 设置环境变量
ENV TZ=Asia/Shanghai

# 安装运行时依赖，添加编程语言支持
RUN apk update && apk add --no-cache \
    bash \
    curl \
    git \
    vim \
    libgcc \
    libstdc++ \
    protobuf \
    gcc \
    g++ \
    python3 \
    python3-dev \
    openjdk11 \
    nodejs \
    npm \
    make \
    cmake

# 创建必要的目录
RUN mkdir -p /workspace/opti-neko/temp && \
    mkdir -p /workspace/opti-neko/submissions && \
    mkdir -p /workspace/opti-neko/testcases

# 创建非特权用户
RUN adduser -D -u 1000 judge

# 从构建阶段复制编译好的二进制文件
COPY --from=builder /workspace/opti-neko/target/release/opti-neko /usr/local/bin/

# 创建工作目录并设置权限
WORKDIR /workspace/opti-neko
RUN chown -R judge:judge /workspace

# 切换到非特权用户
USER judge

# 启动应用程序
CMD ["/usr/local/bin/opti-neko"]
