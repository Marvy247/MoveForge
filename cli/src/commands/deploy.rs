use anyhow::{Result, Context};
use colored::Colorize;
use std::path::Path;
use std::process::Command;
use moveforge_core::MovementClient;

pub async fn run(contract: String, network: String, key: Option<String>) -> Result<()> {
    println!("{}", "🚀 Deploying contract to Movement...".green().bold());
    println!();

    let contract_path = Path::new(&contract);
    if !contract_path.exists() {
        anyhow::bail!("Contract file not found: {}", contract);
    }

    let is_move = contract.ends_with(".move");
    let is_solidity = contract.ends_with(".sol");

    let contract_type = if is_move {
        "Move Module"
    } else if is_solidity {
        "Solidity Contract"
    } else {
        anyhow::bail!("Unknown contract type. Supported: .move, .sol");
    };

    println!("📄 Contract: {}", contract.bright_yellow());
    println!("🔧 Type: {}", contract_type);
    println!("🌐 Network: {}", network.bright_cyan());
    println!();

    // Get Movement client
    let client = match network.as_str() {
        "testnet" => MovementClient::testnet(),
        "mainnet" => MovementClient::mainnet(),
        "local" => MovementClient::local(),
        _ => MovementClient::new(network.clone()),
    };

    println!("📡 Connecting to: {}", client.rpc_url().bright_cyan());
    
    // Health check
    print!("🔍 Checking network connection... ");
    match client.health_check().await {
        Ok(true) => println!("{}", "✓".green()),
        _ => {
            println!("{}", "✗ (network may be unavailable)".yellow());
            println!();
            println!("{}", "⚠️  Showing deployment instructions instead...".yellow());
            println!();
        }
    }
    println!();

    if is_move {
        deploy_move_module(&contract, &network, key, &client).await?;
    } else if is_solidity {
        deploy_solidity_contract(&contract, &network, key).await?;
    }

    Ok(())
}

async fn deploy_move_module(
    contract: &str,
    network: &str,
    key: Option<String>,
    client: &MovementClient,
) -> Result<()> {
    println!("{}", "📦 Move Module Deployment Guide".bright_cyan().bold());
    println!("{}", "═".repeat(70).bright_cyan());
    println!();

    // Check if movement CLI is available
    let movement_check = Command::new("movement")
        .arg("--version")
        .output();

    let aptos_check = Command::new("aptos")
        .arg("--version")
        .output();

    if movement_check.is_ok() {
        println!("{}", "✓ Movement CLI detected".green());
        println!();
        show_movement_cli_instructions(contract, network);
    } else if aptos_check.is_ok() {
        println!("{}", "✓ Aptos CLI detected (Movement compatible)".green());
        println!();
        show_aptos_cli_instructions(contract, network, client.rpc_url());
    } else {
        println!("{}", "⚠️  No CLI detected. Install Movement or Aptos CLI first.".yellow());
        println!();
        show_installation_instructions();
        println!();
        show_movement_cli_instructions(contract, network);
    }

    Ok(())
}

fn show_movement_cli_instructions(contract: &str, network: &str) {
    let module_name = Path::new(contract)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("module");

    println!("{}", "📋 Deployment Steps:".bold());
    println!();
    println!("1️⃣  Create account (if not exists):");
    println!("   {}", "movement account create --account default".bright_yellow());
    println!();
    println!("2️⃣  Fund account from faucet (testnet only):");
    println!("   {}", "movement account fund-with-faucet --account default".bright_yellow());
    println!("   Or visit: {}", "https://faucet.testnet.porto.movementlabs.xyz".bright_cyan());
    println!();
    println!("3️⃣  Deploy your module:");
    println!("   {}", format!("movement move publish --package-dir . --named-addresses {}=default", module_name).bright_yellow());
    println!();
    println!("{}", "═".repeat(70).bright_cyan());
    println!();
    println!("{}", "💡 Note: MoveForge provides testing and simulation before deployment!".bright_green());
    println!("   Run 'moveforge sim' to test your contract first.");
    println!();
}

fn show_aptos_cli_instructions(contract: &str, network: &str, rpc_url: &str) {
    let module_name = Path::new(contract)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("module");

    println!("{}", "📋 Deployment Steps with Aptos CLI:".bold());
    println!();
    println!("1️⃣  Initialize config for Movement:");
    println!("   {}", format!("aptos init --network custom --rest-url {}", rpc_url).bright_yellow());
    println!();
    println!("2️⃣  Get testnet tokens:");
    println!("   Visit: {}", "https://faucet.testnet.porto.movementlabs.xyz".bright_cyan());
    println!();
    println!("3️⃣  Compile and publish:");
    println!("   {}", format!("aptos move compile --named-addresses {}=default", module_name).bright_yellow());
    println!("   {}", format!("aptos move publish --named-addresses {}=default", module_name).bright_yellow());
    println!();
    println!("{}", "═".repeat(70).bright_cyan());
    println!();
}

fn show_installation_instructions() {
    println!("{}", "🔧 Install Movement CLI:".bold());
    println!();
    println!("   Option 1 - Movement CLI (recommended):");
    println!("   {}", "curl -sSf https://raw.githubusercontent.com/movementlabsxyz/aptos-core/main/scripts/dev_setup.sh | bash".bright_yellow());
    println!();
    println!("   Option 2 - Aptos CLI (compatible):");
    println!("   {}", "curl -fsSL \"https://aptos.dev/scripts/install_cli.py\" | python3".bright_yellow());
    println!();
}

async fn deploy_solidity_contract(
    contract: &str,
    network: &str,
    _key: Option<String>,
) -> Result<()> {
    println!("{}", "📦 Solidity Contract Deployment Guide".bright_cyan().bold());
    println!("{}", "═".repeat(70).bright_cyan());
    println!();

    // Check for foundry
    let foundry_check = Command::new("forge")
        .arg("--version")
        .output();

    if foundry_check.is_ok() {
        println!("{}", "✓ Foundry detected".green());
        println!();
        show_foundry_deployment(contract, network);
    } else {
        println!("{}", "⚠️  Foundry not found".yellow());
        println!();
        println!("Install Foundry:");
        println!("   {}", "curl -L https://foundry.paradigm.xyz | bash".bright_yellow());
        println!("   {}", "foundryup".bright_yellow());
        println!();
        show_foundry_deployment(contract, network);
    }

    Ok(())
}

fn show_foundry_deployment(contract: &str, network: &str) {
    let mevm_rpc = if network == "testnet" {
        "https://mevm.testnet.porto.movementlabs.xyz"
    } else {
        "https://mevm.movementnetwork.xyz"
    };

    println!("{}", "📋 Deployment Steps:".bold());
    println!();
    println!("1️⃣  Compile:");
    println!("   {}", "forge build".bright_yellow());
    println!();
    println!("2️⃣  Deploy to Movement EVM:");
    println!("   {}", format!("forge create {} \\", contract).bright_yellow());
    println!("   {}", format!("     --rpc-url {} \\", mevm_rpc).bright_yellow());
    println!("   {}", "     --private-key $PRIVATE_KEY".bright_yellow());
    println!();
    println!("   Or with Hardhat:");
    println!("   {}", format!("npx hardhat run scripts/deploy.js --network movement_{}", network).bright_yellow());
    println!();
    println!("{}", "═".repeat(70).bright_cyan());
    println!();
    println!("{}", "💡 Movement EVM is fully EVM-compatible!".bright_green());
    println!("   Use any Ethereum tool you're familiar with.");
    println!();
}

