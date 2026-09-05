# Bitbucket Model Context Protocol (MCP) Server

[![Build Status](https://github.com/Ibrahimogod/bitbucket-mcp/actions/workflows/ci.yml/badge.svg)](https://github.com/Ibrahimogod/bitbucket-mcp/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/bitbucket-mcp.svg)](https://crates.io/crates/bitbucket-mcp)
[![Rust Version](https://img.shields.io/badge/rust-1.85%2B-blue.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A high-performance, stateless Model Context Protocol (MCP) server written in Rust that brings the full power of the Bitbucket Cloud REST API to your AI agents and LLM automation tools.

By bridging Bitbucket Cloud with the MCP standard, `bitbucket-mcp` allows intelligent agents (like Claude, Cursor, Copilot, or Antigravity) to securely inspect, manage, and automate repositories, pull requests, issues, and CI/CD pipelines autonomously.

---

## 🚀 Features

- **Full Bitbucket Cloud Support**: Interact with Repositories, Workspaces, Branches, Commits, Pull Requests, Issues, Pipelines, Deployments, and Webhooks.
- **MCP Protocol Compliant**: Built strictly on the standard MCP `2024-11-05` protocol specifications.
- **Blazing Fast & Lightweight**: Written in Rust, utilizing highly optimized asynchronous operations and the `rustls` stack for maximum performance and a tiny footprint.
- **Secure by Default**: Completely stateless architecture. Authentication is done via official Bitbucket API tokens passed securely via the environment.
- **Docker-Ready**: Official images are available on the GitHub Container Registry (GHCR) for instant plug-and-play integrations.

---

## 🏗️ Architecture & How It Works

`bitbucket-mcp` utilizes the `rmcp` Rust crate to expose a standard JSON-RPC interface to AI clients. 
When an AI agent requests an action (like retrieving a pull request's diff), the server translates this into a strictly-typed REST API call to `api.bitbucket.org`.

### Strict Schema Validation
Unlike simpler Node.js-based servers, this Rust implementation uses `schemars` to generate rigorous, strongly-typed JSON schemas for its tools. This ensures compatibility with the most strict, enterprise-grade AI LLMs (such as OpenAI's Structured Outputs), avoiding loosely-typed schema errors during tool discovery.

---

## 🛠️ Prerequisites

1. **Bitbucket Credentials**: You need a Bitbucket Cloud account and an **App Password** / API Token.
   - [Create an App Password](https://bitbucket.org/account/settings/app-passwords/) with scopes like `repository:read`, `pullrequest:read`, `pullrequest:write`, `issue:write`, etc.
2. **Docker** (Recommended) or the **Rust toolchain** (if building from source).

---

## 📦 Installation & Quick Start

### Option A: Using Docker (Recommended)

You can run the prebuilt Docker image directly. It accepts JSON-RPC over `stdio`.

```bash
docker run -i --rm \
  -e BITBUCKET_API_USERNAME="your-atlassian-email@example.com" \
  -e BITBUCKET_API_TOKEN="your-app-password" \
  ghcr.io/ibrahimogod/bitbucket-mcp:latest
```

### Option B: Building Locally (Cargo)

If you prefer to run it natively without Docker:

```bash
# 1. Clone the repository
git clone https://github.com/Ibrahimogod/bitbucket-mcp.git
cd bitbucket-mcp

# 2. Build the optimized release binary
cargo build --release

# 3. Export credentials and run
export BITBUCKET_API_USERNAME="your-atlassian-email@example.com"
export BITBUCKET_API_TOKEN="your-app-password"
./target/release/bitbucket_stdio
```

---

## 🔌 Integrating with AI Clients

### Cursor Integration
To configure Cursor to launch the Bitbucket MCP server automatically, add the following to your global `~/.cursor/mcp.json` or project-level `.cursor/mcp.json`:

```json
{
  "mcpServers": {
    "bitbucket-mcp": {
      "command": "docker",
      "args": [
        "run", "-i", "--rm",
        "-e", "BITBUCKET_API_USERNAME",
        "-e", "BITBUCKET_API_TOKEN",
        "ghcr.io/ibrahimogod/bitbucket-mcp:latest"
      ],
      "env": {
        "BITBUCKET_API_USERNAME": "<your-email>",
        "BITBUCKET_API_TOKEN": "<your-app-password>"
      }
    }
  }
}
```

### Claude Desktop Integration
For the Claude Desktop app, edit your configuration file (usually found at `~/Library/Application Support/Claude/claude_desktop_config.json` on macOS):

```json
{
  "mcpServers": {
    "bitbucket": {
      "command": "docker",
      "args": [
        "run", "-i", "--rm",
        "-e", "BITBUCKET_API_USERNAME",
        "-e", "BITBUCKET_API_TOKEN",
        "ghcr.io/ibrahimogod/bitbucket-mcp:latest"
      ],
      "env": {
        "BITBUCKET_API_USERNAME": "<your-email>",
        "BITBUCKET_API_TOKEN": "<your-app-password>"
      }
    }
  }
}
```

---

## 🤝 Contributing

Contributions are welcome! Please ensure that your code adheres to standard Rust formatting (`cargo fmt`) and passes all tests (`cargo test`).

If you'd like to add support for a new Bitbucket Cloud endpoint:
1. Define the input struct in `src/common/bitbucket.rs` using `#[derive(JsonSchema, Deserialize)]`.
2. Add the tool handler logic annotated with `#[rmcp::tool(description = "...")]`.
3. Ensure to add tests mapping the new endpoint.

## 📄 License

This project is licensed under the [MIT License](LICENSE).