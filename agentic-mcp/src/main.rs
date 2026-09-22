use anyhow::Result;
use clap::{Parser, ValueEnum};
use tracing_subscriber::EnvFilter;

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

    // Transport servers and domain services are intentionally introduced behind
    // explicit modules in subsequent milestones. This entrypoint currently
    // validates configuration and provides a stable executable boundary.
    Ok(())
}
