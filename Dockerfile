# Multi-stage Docker build for BollySeerr
# Stage 1: Build the Rust binary
FROM rust:alpine AS builder

WORKDIR /app

RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    build-base

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

COPY src ./src
COPY tests ./tests

RUN touch src/lib.rs src/main.rs && \
    cargo build --release && \
    strip /app/target/release/bollyseerr

# Stage 2: Minimal Alpine runtime image
FROM alpine:3.21

RUN apk add --no-cache \
    ca-certificates \
    libgcc \
    wget

RUN adduser -D -u 568 bollyseerr && \
    mkdir -p /data && \
    chown -R 568:568 /data

WORKDIR /app

COPY --from=builder /app/target/release/bollyseerr /usr/local/bin/bollyseerr

USER 568:568

EXPOSE 5056

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD wget --quiet --tries=1 --spider http://127.0.0.1:5056/health || exit 1

ENTRYPOINT ["/usr/local/bin/bollyseerr"]
