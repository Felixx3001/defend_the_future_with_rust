    # 第一阶段：构建阶段
    # 使用 Rust 官方 Docker 镜像作为基础镜像
    FROM rust:1.74 as builder
    WORKDIR /usr/src/hello_rocket
    # 先复制 Cargo.toml 和 Cargo.lock，并构建一个假的源代码文件
    # 这样做是为了缓存依赖项，不必在每次构建时都重新下载和编译
    COPY Cargo.toml Cargo.lock ./
    RUN mkdir src && echo "fn main() {}" > src/main.rs
    RUN cargo build --release

    # 现在复制真正的源代码并重新构建
    COPY src ./src
    RUN touch src/main.rs
    RUN cargo install --path .

    # 第二阶段：最终镜像
    FROM ubuntu:latest


    # 安装必要的软件包和依赖
    RUN apt-get update && apt-get install -y libc6
    # 从构建阶段复制编译好的应用
    COPY --from=builder /usr/local/cargo/bin/hello_rocket /usr/local/bin/hello_rocket
    ENV ROCKET_ADDRESS=0.0.0.0


    # 设置容器启动时执行的命令
    CMD ["hello_rocket"]
