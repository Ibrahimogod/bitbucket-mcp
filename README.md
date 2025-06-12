# Bitbucket MCP Server

**Bitbucket MCP** is a high-performance, stateless server that brings the full power of the Bitbucket Cloud API to your automation, CI/CD pipelines, bots, and Rust-based integrations. Built in Rust for speed and reliability, Bitbucket MCP makes it easy to securely access and manage Bitbucket repositories, pull requests, issues, and more—whether you're building developer tools, workflow automation, or DevOps solutions.

---

## Why Bitbucket MCP?
- **Seamless Bitbucket API Integration**: Access all major Bitbucket Cloud features—repositories, pull requests, issues, branches, pipelines, deployments, and more—using a modern Rust codebase.
- **Perfect for Automation & Bots**: Expose Bitbucket as Model Context Protocol (MCP) tools, ideal for bots, CI/CD, and workflow automation.
- **Secure by Default**: Uses only `rustls` for TLS (no OpenSSL headaches), and supports Bitbucket App Password authentication.
- **Docker-Ready**: Deploy anywhere with our prebuilt Docker images on GHCR, or build locally in minutes.
- **Battle-Tested**: Comprehensive test suite covers all public API methods, ensuring reliability for your integrations.

---

## Quick Start: Bitbucket API Automation in Rust

### 1. Use the Prebuilt Docker Image from GHCR

```sh
docker run -e BITBUCKET_API_USERNAME=<your_username> -e BITBUCKET_APP_PASSWORD=<your_app_password> -p 8080:8080 ghcr.io/ibrahimogod/bitbucket-mcp:latest
```

- Find all tags/releases at: [GitHub Releases](https://github.com/Ibrahimogod/bitbucket-mcp/releases)
- See [GHCR package](https://github.com/users/Ibrahimogod/packages/container/bitbucket-mcp)

### 2. Build and Run Locally (Rust)

```sh
git clone https://github.com/Ibrahimogod/bitbucket-mcp.git
cd bitbucket-mcp
cargo build --release
$env:BITBUCKET_API_USERNAME="<your_username>"
$env:BITBUCKET_APP_PASSWORD="<your_app_password>"
cargo run --release --bin bitbucket_stdio
```

---

## Supported Bitbucket Operations (via MCP)
- List and manage repositories, workspaces, pull requests, issues, branches, tags, commits
- Get repository, workspace, and user details
- Automate pull request workflows: create, update, approve, decline, merge, comment, and manage tasks
- Integrate with Bitbucket pipelines, deployments, downloads, webhooks, snippets, and projects
- See [`src/common/bitbucket.rs`](src/common/bitbucket.rs) for the full API

---

## Bitbucket Authentication
- [Create a Bitbucket App Password](https://bitbucket.org/account/settings/app-passwords/) with `Repository:Read` and `Pull requests:Read` permissions.
- Set `BITBUCKET_API_USERNAME` to your Atlassian email.
- Set `BITBUCKET_APP_PASSWORD` to your App Password.

---

## Project Structure
- `src/common/bitbucket.rs` — Bitbucket API integration logic
- `src/bitbucket_stdio.rs` — Server entry point
- `Cargo.toml` — Dependency configuration (uses `rustls` only)
- `Dockerfile` — Multi-stage build, no OpenSSL
- `tests/bitbucket.rs` — Full test suite for all public API methods

---

## License
MIT

---

**Bitbucket MCP** is the best way to automate Bitbucket Cloud with Rust, bots, or CI/CD. Star the repo and try it today!