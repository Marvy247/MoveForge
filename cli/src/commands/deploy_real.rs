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
    let health = client.health_check().await?;
    if health {
        println!("{}", "✓".green());
    } else {
        anyhow::bail!("Network not reachable");
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
    println!("{}", "📦 Compiling Move module...".yellow());
    
    // Check if movement CLI is available
    let movement_check = Command::new("movement")
        .arg("--version")
        .output();

    if movement_check.is_err() {
        println!("{}", "⚠️  Movement CLI not found. Using aptos CLI instead...".yellow());
        return deploy_with_aptos_cli(contract, network, key).await;
    }

    println!("{}", "⚠️  Note: For production deployment, use Movement CLI with your account".yellow());
    println!("{}", "Example: movement move publish --package-dir . --named-addresses moveforge_nft=default".yellow());
    println!();

    // For demo purposes, show what would be deployed
    println!("{}", "📋 Contract Summary:".bright_cyan().bold());
    println!("   Contract: {}", contract);
    println!("   Network: {}", network);
    println!("   RPC: {}", client.rpc_url());
    println!();

    println!("{}", "✅ To deploy this contract:".green().bold());
    println!();
    println!("1. Install Movement CLI:");
    println!("   {}", "curl -sSf https://raw.githubusercontent.com/movementlabsxyz/aptos-core/main/scripts/dev_setup.sh | bash".bright_yellow());
    println!();
    println!("2. Create an account:");
    println!("   {}", "movement account create --account default".bright_yellow());
    println!();
    println!("3. Fund your account (testnet):");
    println!("   {}", "movement account fund-with-faucet --account default".bright_yellow());
    println!();
    println!("4. Deploy the module:");
    let module_name = Path::new(contract)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("module");
    println!("   {}", format!("movement move publish --package-dir . --named-addresses {}=default", module_name).bright_yellow());
    println!();

    Ok(())
}

async fn deploy_with_aptos_cli(
    contract: &str,
    network: &str,
    _key: Option<String>,
) -> Result<()> {
    println!("{}", "Using Aptos CLI (Movement compatible)...".cyan());
    
    // Check if aptos CLI is available
    let aptos_check = Command::new("aptos")
        .arg("--version")
        .output();

    if aptos_check.is_err() {
        println!("{}", "⚠️  Aptos CLI not found either.".yellow());
        return show_manual_deployment_instructions(contract, network);
    }

    println!("{}", "✅ Aptos CLI found!".green());
    println!();
    println!("To deploy with Aptos CLI:");
    println!();
    println!("1. Initialize your account:");
    println!("   {}", "aptos init --network custom --rest-url https://aptos.testnet.porto.movementlabs.xyz/v1".bright_yellow());
    println!();
    println!("2. Get testnet tokens:");
    println!("   Visit: https://faucet.testnet.porto.movementlabs.xyz".bright_cyan());
    println!();
    println!("3. Compile and publish:");
    let module_name = Path::new(contract)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("module");
    println!("   {}", format!("aptos move publish --named-addresses {}=default", module_name).bright_yellow());
    println!();

    Ok(())
}

async fn deploy_solidity_contract(
    contract: &str,
    network: &str,
    _key: Option<String>,
) -> Result<()> {
    println!("{}", "📦 Compiling Solidity contract...".yellow());
    
    // Check for foundry
    let foundry_check = Command::new("forge")
        .arg("--version")
        .output();

    if foundry_check.is_ok() {
        println!("{}", "✓ Foundry found".green());
        println!();
        println!("To deploy Solidity on Movement EVM:");
        println!();
        println!("1. Compile:");
        println!("   {}", "forge build".bright_yellow());
        println!();
        println!("2. Deploy to Movement EVM:");
        println!("   {}", format!("forge create {} --rpc-url https://mevm.testnet.porto.movementlabs.xyz --private-key $PRIVATE_KEY", contract).bright_yellow());
        println!();
    } else {
        println!("{}", "⚠️  Foundry not found. Install: https://getfoundry.sh".yellow());
        println!();
        println!("Movement supports EVM contracts via the MEVM layer.");
        println!("Use standard Ethereum tools (Foundry, Hardhat) to deploy.");
        println!();
        println!("Movement EVM RPC: {}", "https://mevm.testnet.porto.movementlabs.xyz".bright_cyan());
    }

    Ok(())
}

fn show_manual_deployment_instructions(contract: &str, network: &str) -> Result<()> {
    println!();
    println!("{}", "📚 Manual Deployment Guide".bright_cyan().bold());
    println!("{}", "═".repeat(60).bright_cyan());
    println!();
    println!("Your Move contract: {}", contract.bright_yellow());
    println!("Target network: {}", network.bright_cyan());
    println!();
    println!("{}", "Step-by-step deployment:".bold());
    println!();
    println!("1️⃣  Install Movement CLI:");
    println!();
    println!("   curl -sSf https://raw.githubusercontent.com/movementlabsxyz/aptos-core/main/scripts/dev_setup.sh | bash");
    println!("   source ~/.profile");
    println!();
    println!("2️⃣  Create and fund account:");
    println!();
    println!("   movement account create --account default");
    println!("   movement account fund-with-faucet --account default");
    println!();
    println!("3️⃣  Prepare your Move package:");
    println!();
    println!("   Create a Move.toml file in your project root:");
    println!();
    println!("   [package]");
    println!("   name = \"MoveForgeExample\"");
    println!("   version = \"1.0.0\"");
    println!();
    println!("   [addresses]");
    println!("   moveforge_nft = \"_\"");
    println!();
    println!("   [dependencies.AptosFramework]");
    println!("   git = \"https://github.com/aptos-labs/aptos-core.git\"");
    println!("   subdir = \"aptos-move/framework/aptos-framework\"");
    println!("   rev = \"mainnet\"");
    println!();
    println!("4️⃣  Deploy:");
    println!();
    let module_name = Path::new(contract)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("module");
    println!("   movement move publish --named-addresses {}=default", module_name);
    println!();
    println!("{}", "═".repeat(60).bright_cyan());
    println!();
    println!("{}", "💡 Pro tip: MoveForge will handle this automatically in future releases!".bright_green());
    println!();

    Ok(())
}
