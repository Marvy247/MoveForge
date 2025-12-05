use anyhow::Result;
use colored::Colorize;
use moveforge_core::node::LocalNode;
use tracing::info;

pub async fn run(fork: Option<String>, port: u16, parallel: bool) -> Result<()> {
    println!("{}", "Starting MoveForge local node...".green());
    
    let fork_url = if let Some(network) = fork {
        let url = match network.as_str() {
            "testnet" => "https://aptos.testnet.porto.movementlabs.xyz/v1",
            "mainnet" => "https://mainnet.movementnetwork.xyz/v1",
            custom => custom,
        };
        info!("Forking from: {}", url);
        println!("📡 Forking from: {}", url.bright_yellow());
        Some(url.to_string())
    } else {
        println!("🏗️  Starting fresh local node (no fork)");
        None
    };

    println!("⚡ Parallel execution: {}", if parallel { "ENABLED".green() } else { "DISABLED".red() });
    println!("🌐 RPC endpoint: {}", format!("http://localhost:{}", port).bright_cyan());
    println!();

    let node = LocalNode::new(port, fork_url, parallel).await?;
    
    println!("{}", "✅ Node is running! Press Ctrl+C to stop.".bright_green().bold());
    println!();
    println!("Connect your app to: {}", format!("http://localhost:{}", port).bright_cyan());
    println!();
    
    node.run().await?;
    
    Ok(())
}
