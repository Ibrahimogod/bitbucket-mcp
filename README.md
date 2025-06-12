# Bitbucket MCP Server

This project provides a Model Context Protocol (MCP) server for Bitbucket integration, implemented in Rust. It enables secure, stateless access to Bitbucket repositories, pull requests, and related resources, suitable for automation and integration scenarios.

## Features
- **Bitbucket API Integration**: List repositories, fetch repository details, and list pull requests via Bitbucket Cloud REST API.
- **Stateless HTTP API**: Exposes endpoints for MCP clients to interact with Bitbucket.
- **Secure by Default**: Uses only `rustls` for TLS (no OpenSSL dependency).
- **Dockerized**: Build and run easily in a containerized environment.
- **Prebuilt Image on GHCR**: Use the latest Docker image from GitHub Container Registry (GHCR), built automatically on every push to `main`.

## Requirements
- Rust (for local builds)
- Docker (for containerized builds and deployment)
- Bitbucket App Password (recommended) or Bitbucket username and app password (see below)

## Required Environment Variables
- `BITBUCKET_API_USERNAME`: Your Bitbucket/Atlassian email address (used for API authentication)
- `BITBUCKET_APP_PASSWORD`: Your Bitbucket App Password (see below)
- `RUST_BACKTRACE`: (Optional) Set to `1` for backtraces on errors

## Usage

### 1. Use the Prebuilt Docker Image from GHCR

```powershell
# Pull and run the image for a specific commit (recommended for reproducibility)
# Replace <sha> with the 7-character commit hash from the GitHub Release or commit log
# Replace <your_username> and <your_app_password> with your Bitbucket credentials

docker run -e BITBUCKET_API_USERNAME=<your_username> -e BITBUCKET_APP_PASSWORD=<your_app_password> -p 8080:8080 ghcr.io/ibrahimogod/bitbucket-mcp:<sha>
```

- Or use `:latest` for the most recent build (not recommended for production):

```powershell
docker run -e BITBUCKET_API_USERNAME=<your_username> -e BITBUCKET_APP_PASSWORD=<your_app_password> -p 8080:8080 ghcr.io/ibrahimogod/bitbucket-mcp:latest
```

- The image is published at: [ghcr.io/ibrahimogod/bitbucket-mcp](https://github.com/users/Ibrahimogod/packages/container/bitbucket-mcp)
- **Find the correct `<sha>` tag**: Go to the [GitHub Releases page](https://github.com/Ibrahimogod/bitbucket-mcp/releases) and use the 7-character tag for the release you want.

### 2. Build and Run Locally with Docker

```powershell
# Build the Docker image (no OpenSSL required)
docker build --no-cache -t bitbucket-mcp-rustls-only .

docker run -e BITBUCKET_API_USERNAME=<your_username> -e BITBUCKET_APP_PASSWORD=<your_app_password> -p 8080:8080 bitbucket-mcp-rustls-only
```

### 3. Local Development

```powershell
$env:BITBUCKET_API_USERNAME="<your_username>"
$env:BITBUCKET_APP_PASSWORD="<your_app_password>"
cargo run --release --bin bitbucket_stdio
```

## Available MCP Tool Operations
This MCP server exposes Bitbucket operations as MCP tools, not as HTTP endpoints. You can use these operations via an MCP-compatible client:
- Get Bitbucket user info
- List Bitbucket workspaces
- List repositories in a workspace
- List pull requests for a repository
- List issues for a repository
- Get workspace details
- Get repository details
- List branches, tags, and commits for a repository
- List pipelines, deployments, downloads, webhooks, snippets, projects, branch restrictions, commit statuses, and users

See `src/common/bitbucket.rs` for the full list of supported operations and their parameters.

## Bitbucket Authentication
- **Recommended**: [Create a Bitbucket App Password](https://bitbucket.org/account/settings/app-passwords/) with at least `Repository:Read` and `Pull requests:Read` permissions.
- **Username**: Use your Atlassian email as `BITBUCKET_API_USERNAME`.
- **App Password**: Use the generated App Password as `BITBUCKET_APP_PASSWORD`.

## TLS/Dependency Notes
- The project is configured to use only `rustls` for TLS (see `Cargo.toml`).
- No OpenSSL or `native-tls` dependencies are present in the build or Docker image.

## Project Structure
- `src/common/bitbucket.rs` — Bitbucket API integration logic
- `src/bitbucket_stdio.rs` — Server entry point
- `Cargo.toml` — Dependency configuration (uses `rustls` only)
- `Dockerfile` — Multi-stage build, no OpenSSL

## Troubleshooting
- **Authentication errors**: Ensure your username and App Password are correct and have the required permissions.
- **OpenSSL errors**: Should not occur; if they do, check that `Cargo.toml` disables default features for `reqwest` and enables only `rustls-tls`.

## License
MIT