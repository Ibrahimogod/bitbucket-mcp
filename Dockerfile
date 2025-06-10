# Build MCP Bitbucket server
FROM rust:1.87 as builder
WORKDIR /app
# Install musl-tools for static linking
RUN apt-get update && apt-get install -y musl-tools && rm -rf /var/lib/apt/lists/*
# Optimize Docker cache by copying only Cargo files first
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --target x86_64-unknown-linux-musl || true
# Now copy the rest of the source
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl && strip target/x86_64-unknown-linux-musl/release/bitbucket-mcp

# Runtime image (Alpine for minimal size)
FROM alpine:3.20
WORKDIR /app
RUN adduser -D appuser
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/bitbucket-mcp /app/bitbucket-mcp
RUN chown appuser:appuser /app/bitbucket-mcp
USER appuser
ENV PORT=8080
EXPOSE 8080
CMD ["/app/bitbucket-mcp"]
