FROM rust:1.87.0-alpine AS builder
WORKDIR /code

# Install build dependencies for Rust (no OpenSSL needed for rustls-tls)
RUN apk add --no-cache musl-dev pkgconfig build-base

RUN USER=root cargo init

COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo fetch

COPY src src

RUN cargo build --release --bin bitbucket_stdio

FROM alpine:3.20
WORKDIR /app

RUN apk add --no-cache libgcc libstdc++ ca-certificates

COPY --from=builder /code/target/release/bitbucket_stdio bitbucket_mcp

USER 1001

EXPOSE 8080

CMD ["/app/bitbucket_mcp"]