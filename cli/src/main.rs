use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod commands;

#[derive(Parser)]
#[command(name = "moveforge")]
#[command(about = "MoveForge - Foundry-grade testing toolkit for Movement blockchain", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a local ParaForge node with optional forking
    Node {
        /// Fork from Movement testnet or mainnet
        #[arg(long)]
        fork: Option<String>,
        
        /// Port to run the node on
        #[arg(long, default_value = "8545")]
        port: u16,
        
        /// Enable parallel execution mode
        #[arg(long, default_value = "true")]
        parallel: bool,
    },
    
    /// Simulate transactions in parallel
    Sim {
        /// Run simulations in parallel
        #[arg(long, default_value = "true")]
        parallel: bool,
        
        /// Transaction file to simulate (JSON)
        #[arg(short, long)]
        file: Option<String>,
        
        /// Number of concurrent transactions
        #[arg(short, long, default_value = "10")]
        count: usize,
        
        /// Output format (json, table, web)
        #[arg(short, long, default_value = "table")]
        output: String,
        
        /// Launch web dashboard
        #[arg(long)]
        web: bool,
    },
    
    /// Run test suite with fuzzing and invariants
    Test {
        /// Path to test files or directory
        path: Option<String>,
        
        /// Enable fuzzing
        #[arg(long)]
        fuzz: bool,
        
        /// Number of fuzz iterations
        #[arg(long, default_value = "1000")]
        fuzz_runs: usize,
        
        /// Enable parallel test execution
        #[arg(long, default_value = "true")]
        parallel: bool,
    },
    
    /// Deploy contracts to Movement network
    Deploy {
        /// Contract path (Move module or Solidity)
        contract: String,
        
        /// Network (testnet, mainnet, local)
        #[arg(short, long, default_value = "testnet")]
        network: String,
        
        /// Private key or wallet path
        #[arg(short, long)]
        key: Option<String>,
    },
    
    /// Analyze transactions with AI insights
    Analyze {
        /// Transaction hash or simulation result file
        input: String,
        
        /// Enable vulnerability detection
        #[arg(long)]
        vuln: bool,
        
        /// Enable gas optimization suggestions
        #[arg(long)]
        optimize: bool,
    },
    
    /// Initialize a new ParaForge project
    Init {
        /// Project name
        name: String,
        
        /// Template (move, solidity, hybrid)
        #[arg(short, long, default_value = "hybrid")]
        template: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "moveforge=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    println!("{}", "⚡ MoveForge - Movement DevEx Toolkit".bright_cyan().bold());
    println!();

    match cli.command {
        Commands::Node { fork, port, parallel } => {
            commands::node::run(fork, port, parallel).await?;
        }
        Commands::Sim { parallel, file, count, output, web } => {
            commands::sim::run(parallel, file, count, output, web).await?;
        }
        Commands::Test { path, fuzz, fuzz_runs, parallel } => {
            commands::test::run(path, fuzz, fuzz_runs, parallel).await?;
        }
        Commands::Deploy { contract, network, key } => {
            commands::deploy::run(contract, network, key).await?;
        }
        Commands::Analyze { input, vuln, optimize } => {
            commands::analyze::run(input, vuln, optimize).await?;
        }
        Commands::Init { name, template } => {
            commands::init::run(name, template).await?;
        }
    }

    Ok(())
}
