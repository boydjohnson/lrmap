FROM ghcr.io/cross-rs/aarch64-unknown-linux-gnu:latest

RUN dpkg --add-architecture arm64 && \
    apt-get update && \
    apt-get upgrade -y && \
    apt-get install --assume-yes \
    protobuf-compiler \
    libsqlite3-dev:arm64 \
    sqlite3 \
    libclang-5.0-dev \
    clang
