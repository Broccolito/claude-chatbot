use anyhow::Result;
use clap::Parser;
use std::env;

mod api;
mod ui;
mod artifacts;
mod mcp;
mod markdown;

use api::ClaudeClient;
use ui::ChatApp;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Anthropic API key (or set ANTHROPIC_API_KEY environment variable)
    #[arg(short, long)]
    api_key: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    let api_key = args.api_key
        .or_else(|| env::var("ANTHROPIC_API_KEY").ok())
        .ok_or_else(|| anyhow::anyhow!("API key required. Use --api-key or set ANTHROPIC_API_KEY"))?;

    let client = ClaudeClient::new(api_key);
    let mut app = ChatApp::new(client);
    
    app.run().await?;
    
    Ok(())
}

