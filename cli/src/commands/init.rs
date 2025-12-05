use anyhow::Result;
use colored::Colorize;
use std::fs;
use std::path::Path;

pub async fn run(name: String, template: String) -> Result<()> {
    println!("{}", "Initializing new MoveForge project...".green());
    println!();

    let project_path = Path::new(&name);
    
    if project_path.exists() {
        anyhow::bail!("Directory '{}' already exists", name);
    }

    println!("📦 Project name: {}", name.bright_cyan());
    println!("📋 Template: {}", template.bright_yellow());
    println!();

    fs::create_dir_all(project_path.join("contracts"))?;
    fs::create_dir_all(project_path.join("tests"))?;
    fs::create_dir_all(project_path.join("scripts"))?;

    let config = match template.as_str() {
        "move" => generate_move_template(&name),
        "solidity" => generate_solidity_template(&name),
        "hybrid" => generate_hybrid_template(&name),
        _ => anyhow::bail!("Unknown template: {}", template),
    };

    fs::write(project_path.join("moveforge.toml"), config)?;

    let test_content = r#"// Sample test file
// Run with: paraforge test

#[test]
async fn test_transfer() {
    // Your test code here
}
"#;
    fs::write(project_path.join("tests/test_example.rs"), test_content)?;

    let readme = format!(r#"# {}

A MoveForge project for Movement blockchain.

## Setup

```bash
cd {}
moveforge node --fork testnet
```

## Test

```bash
moveforge test
```

## Deploy

```bash
moveforge deploy contracts/YourContract.move --network testnet
```

## Simulate

```bash
moveforge sim --parallel --count 100 --web
```
"#, name, name);

    fs::write(project_path.join("README.md"), readme)?;

    println!("{}", "✅ Project initialized successfully!".bright_green().bold());
    println!();
    println!("Next steps:");
    println!("  cd {}", name.bright_cyan());
    println!("  paraforge node --fork testnet");
    println!();

    Ok(())
}

fn generate_move_template(name: &str) -> String {
    format!(r#"[project]
name = "{}"
version = "0.1.0"
template = "move"

[dependencies]
movement-stdlib = "latest"

[network.testnet]
rpc = "https://aptos.testnet.porto.movementlabs.xyz/v1"

[simulation]
parallel = true
max_concurrent = 100
"#, name)
}

fn generate_solidity_template(name: &str) -> String {
    format!(r#"[project]
name = "{}"
version = "0.1.0"
template = "solidity"

[dependencies]

[network.testnet]
rpc = "https://aptos.testnet.porto.movementlabs.xyz/v1"

[simulation]
parallel = true
max_concurrent = 100
"#, name)
}

fn generate_hybrid_template(name: &str) -> String {
    format!(r#"[project]
name = "{}"
version = "0.1.0"
template = "hybrid"

[dependencies]
movement-stdlib = "latest"

[network.testnet]
rpc = "https://aptos.testnet.porto.movementlabs.xyz/v1"

[simulation]
parallel = true
max_concurrent = 100

[hybrid]
move_to_evm = true
evm_to_move = true
"#, name)
}
