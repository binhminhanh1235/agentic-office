use anyhow::Result;
use clap::{Parser, ValueEnum};
use tracing_subscriber::EnvFilter;

mod domain;
mod policy;

#[derive(Debug, Clone, ValueEnum)]
enum Transport {
    Stdio,
    Http,
    Both,
}

#[derive(Debug, Parser)]
#[command(name = "agentic-office-mcp")]
#[command(about = "Controlled MCP gateway for Agentic Office documents")]
struct Args {
    #[arg(long, value_enum, default_value_t = Transport::Both)]
    transport: Transport,

    #[arg(long, env = "AGENTIC_OFFICE_WORKSPACE")]
    workspace: Option<String>,

    #[arg(long, default_value = "127.0.0.1:8787")]
    http_bind: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    tracing::info!(?args.transport, ?args.workspace, %args.http_bind, "starting Agentic Office MCP foundation");

    // Protocol transports and concrete document services are introduced in
    // subsequent milestones. Domain and policy modules are now explicit so
    // transport implementations can share the same safety contracts.
    Ok(())
}
