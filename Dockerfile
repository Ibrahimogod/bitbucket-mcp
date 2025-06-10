# Build MCP Bitbucket server
FROM rust:1.87 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime image
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/bitbucket-mcp /app/bitbucket-mcp
ENV PORT=8080
EXPOSE 8080
CMD ["/app/bitbucket-mcp"]
