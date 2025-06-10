mod bitbucket;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::env;
use bitbucket::repo::{bitbucket_repo_handler, bitbucket_list_repos_handler, BitbucketAuth};
use bitbucket::proxy::bitbucket_proxy_handler;
use bitbucket::pull_request::bitbucket_pull_request_handler;

#[derive(serde::Serialize)]
struct MCPInfo {
    name: String,
    version: String,
    description: String,
    protocol: String,
}

#[derive(serde::Serialize)]
struct MCPHealth {
    status: String,
}

#[derive(serde::Serialize)]
struct MCPContext {
    name: String,
    version: String,
    description: String,
    // Add more fields as required by your MCP context
}

async fn mcp_info() -> impl Responder {
    web::Json(MCPInfo {
        name: "bitbucket-mcp".to_string(),
        version: "0.1.0".to_string(),
        description: "MCP server for Bitbucket integration".to_string(),
        protocol: "model-context-protocol/1.0".to_string(),
    })
}

async fn mcp_health() -> impl Responder {
    web::Json(MCPHealth {
        status: "ok".to_string(),
    })
}

async fn mcp_context() -> impl Responder {
    web::Json(MCPContext {
        name: "bitbucket-mcp".to_string(),
        version: "0.1.0".to_string(),
        description: "Bitbucket context provider for MCP".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bitbucket_token = env::var("BITBUCKET_TOKEN").unwrap_or_else(|_| "".to_string());
    let auth_data = web::Data::new(BitbucketAuth {
        token: bitbucket_token,
    });
    println!("Starting MCP server on 0.0.0.0:{}", port);
    HttpServer::new(move || {
        App::new()
            .app_data(auth_data.clone())
            .route("/mcp/info", web::get().to(mcp_info))
            .route("/mcp/health", web::get().to(mcp_health))
            .route("/mcp/context", web::get().to(mcp_context))
            .route("/bitbucket/repo", web::post().to(bitbucket_repo_handler))
            .route("/bitbucket/list_repos", web::post().to(bitbucket_list_repos_handler))
            .route("/bitbucket/proxy", web::post().to(bitbucket_proxy_handler))
            .route("/bitbucket/pull_request", web::post().to(bitbucket_pull_request_handler))
    })
    .bind(("0.0.0.0", port.parse().unwrap()))?
    .run()
    .await
}
