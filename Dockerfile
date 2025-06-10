FROM rust:1.87.0-alpine AS builder
WORKDIR /code

# Install build dependencies for Rust and OpenSSL
RUN apk add --no-cache musl-dev openssl-dev pkgconfig build-base

COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo fetch

COPY src src

RUN cargo build --release

FROM alpine:3.20
WORKDIR /app

# Install runtime dependencies for OpenSSL
RUN apk add --no-cache libgcc libstdc++ ca-certificates openssl

COPY --from=builder /code/target/release/bitbucket-mcp bitbucket-mcp

USER 1001

EXPOSE 8080

CMD ["/app/bitbucket-mcp"]