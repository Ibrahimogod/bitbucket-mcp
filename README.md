# Bitbucket MCP Server

This project provides a Model Context Protocol (MCP) server for Bitbucket integration, implemented in Rust. It enables secure, stateless access to Bitbucket repositories, pull requests, and related resources, suitable for automation and integration scenarios.

## Features
- **Bitbucket API Integration**: List repositories, fetch repository details, and list pull requests via Bitbucket Cloud REST API.
- **Stateless HTTP API**: Exposes endpoints for MCP clients to interact with Bitbucket.
- **Secure by Default**: Uses only `rustls` for TLS (no OpenSSL dependency).
- **Dockerized**: Build and run easily in a containerized environment.

## Requirements
- Rust (for local builds)
- Docker (for containerized builds and deployment)
- Bitbucket App Password or OAuth token (see below)

## Usage

### 1. Build and Run with Docker

```powershell
# Build the Docker image (no OpenSSL required)
docker build --no-cache -t bitbucket-mcp-rustls-only .

# Run the container (replace <TOKEN> with your Bitbucket App Password or OAuth token)
docker run -e BITBUCKET_TOKEN=<TOKEN> -p 8080:8080 bitbucket-mcp-rustls-only
```

### 2. Environment Variables
- `BITBUCKET_TOKEN`: Your Bitbucket App Password or OAuth token (App Password recommended for most use cases).
- `RUST_BACKTRACE`: (Optional) Set to `1` for backtraces on errors.

### 3. Endpoints
- `POST /bitbucket/repo` — Get repository details
- `POST /bitbucket/list_repos` — List repositories in a workspace
- `POST /bitbucket/pull_request` — List or get pull requests
- `POST /bitbucket/proxy` — Proxy arbitrary Bitbucket API requests

See `src/bitbucket/` for request/response formats.

## Bitbucket Authentication
- **Recommended**: [Create a Bitbucket App Password](https://bitbucket.org/account/settings/app-passwords/) with at least `Repository:Read` and `Pull requests:Read` permissions.
- **Token Format**: Use the App Password as the value for `BITBUCKET_TOKEN`.
- **OAuth**: If using OAuth, ensure the token is valid and has the required scopes.

## TLS/Dependency Notes
- The project is configured to use only `rustls` for TLS (see `Cargo.toml`).
- No OpenSSL or `native-tls` dependencies are present in the build or Docker image.

## Development

### Local Build
```powershell
cargo build --release
```

### Running Locally
```powershell
$env:BITBUCKET_TOKEN="<TOKEN>"
cargo run --release --bin bitbucket_stdio
```

## Project Structure
- `src/bitbucket/` — Bitbucket API handlers (repo, pull_request, proxy)
- `src/main.rs` — Server entry point
- `Cargo.toml` — Dependency configuration (uses `rustls` only)
- `Dockerfile` — Multi-stage build, no OpenSSL

## Troubleshooting
- **Token errors**: Ensure your App Password is valid, not expired, and has the correct permissions.
- **OpenSSL errors**: Should not occur; if they do, check that `Cargo.toml` disables default features for `reqwest` and enables only `rustls-tls`.

## License
MIT