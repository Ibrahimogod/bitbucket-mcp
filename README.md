# Bitbucket MCP

A minimal, production-ready Rust service for Bitbucket MCP, containerized with Docker and optimized for small image size and fast build times.

## Prerequisites
- Docker (recommended for running in production)
- Rust toolchain (if running locally without Docker)

## Environment Variables
Create a `.env` file or set these variables in your deployment environment:

```
# .env.sample
PORT=8080
# Add other environment variables as needed for your application
```

## Running with Docker

1. Build the Docker image:
   ```sh
   docker build -t bitbucket-mcp:latest .
   ```

2. Run the container:
   ```sh
   docker run --rm -p 8080:8080 --env-file .env bitbucket-mcp:latest
   ```

## Running Locally (without Docker)

1. Install Rust (if not already):
   https://rustup.rs/

2. Build and run:
   ```sh
   cargo run --release
   ```

3. The service will be available at `http://localhost:8080` (or the port you set in the `PORT` env variable).

---

For more configuration, see the code and update `.env.sample` as needed.
