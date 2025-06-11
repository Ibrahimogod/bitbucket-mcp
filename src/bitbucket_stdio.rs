use anyhow::Result;
use common::bitbucket::BitbucketTool;
use rmcp::{ServiceExt, transport::stdio};
use tracing_subscriber::{self, filter::EnvFilter};
mod common;

#[tokio::main]
async fn main() -> Result<()> {
    std::panic::set_hook(Box::new(|panic_info| {
        let msg = match panic_info.payload().downcast_ref::<&str>() {
            Some(s) => *s,
            None => "Unknown panic",
        };
        let location = panic_info.location().map(|l| l.to_string()).unwrap_or_default();
        eprintln!("PANIC: {} at {}", msg, location);
        if let Ok(bt) = std::env::var("RUST_BACKTRACE") {
            if bt == "1" {
                let bt = std::backtrace::Backtrace::force_capture();
                eprintln!("Backtrace:\n{:?}", bt);
            }
        }
    }));
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::WARN.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting Bitbucket MCP server");

    let service = BitbucketTool.serve(stdio()).await.inspect_err(|e| {
        tracing::error!("serving error: {:?}", e);
    })?;

    service.waiting().await?;
    Ok(())
}
