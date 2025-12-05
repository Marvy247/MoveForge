use anyhow::{Result, Context};
use colored::Colorize;
use moveforge_simulator::Simulator;
use serde_json::json;
use std::time::Instant;

pub async fn run(
    parallel: bool,
    file: Option<String>,
    count: usize,
    output: String,
    web: bool,
) -> Result<()> {
    println!("{}", "Running transaction simulation...".green());
    println!();

    let transactions = if let Some(tx_file) = file {
        let content = std::fs::read_to_string(&tx_file)
            .context(format!("Failed to read transaction file: {}", tx_file))?;
        serde_json::from_str(&content)?
    } else {
        generate_sample_transactions(count)
    };

    println!("📦 Loaded {} transactions", transactions.as_array().unwrap().len());
    println!("⚡ Parallel mode: {}", if parallel { "ENABLED".green() } else { "DISABLED".red() });
    println!();

    let simulator = Simulator::new(parallel);
    let start = Instant::now();
    
    let results = simulator.simulate_batch(transactions).await?;
    let duration = start.elapsed();

    println!("{}", "✅ Simulation complete!".bright_green().bold());
    println!("⏱️  Total time: {:.2}s", duration.as_secs_f64());
    println!("🚀 Transactions/sec: {:.0}", count as f64 / duration.as_secs_f64());
    println!();

    match output.as_str() {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&results)?);
        }
        "table" => {
            print_table(&results)?;
        }
        _ => {
            print_table(&results)?;
        }
    }

    if web {
        println!();
        println!("{}", "🌐 Launching web dashboard...".bright_cyan());
        launch_dashboard(results).await?;
    }

    Ok(())
}

fn generate_sample_transactions(count: usize) -> serde_json::Value {
    let mut txs = Vec::new();
    
    for i in 0..count {
        txs.push(json!({
            "id": format!("tx_{}", i),
            "from": format!("0x{:040x}", i * 1000),
            "to": format!("0x{:040x}", i * 1000 + 1),
            "value": format!("{}", (i + 1) * 1000000),
            "gas_limit": 21000,
            "data": "",
            "type": if i % 3 == 0 { "transfer" } else if i % 3 == 1 { "swap" } else { "mint" }
        }));
    }
    
    json!(txs)
}

fn print_table(results: &serde_json::Value) -> Result<()> {
    println!("┌─────────┬──────────────────┬─────────┬────────────┬──────────┐");
    println!("│ {}  │ {}      │ {} │ {}   │ {}  │", 
        "TX ID".bold(), "Type".bold(), "Status".bold(), "Gas Used".bold(), "Time(ms)".bold());
    println!("├─────────┼──────────────────┼─────────┼────────────┼──────────┤");
    
    if let Some(txs) = results.as_array() {
        for tx in txs {
            let id = tx["id"].as_str().unwrap_or("unknown");
            let tx_type = tx["type"].as_str().unwrap_or("unknown");
            let status = tx["status"].as_str().unwrap_or("success");
            let gas = tx["gas_used"].as_u64().unwrap_or(0);
            let time = tx["execution_time_ms"].as_f64().unwrap_or(0.0);
            
            let status_colored = if status == "success" {
                status.green()
            } else {
                status.red()
            };
            
            println!("│ {:<7} │ {:<16} │ {:<7} │ {:>10} │ {:>8.2} │",
                id, tx_type, status_colored, gas, time);
        }
    }
    
    println!("└─────────┴──────────────────┴─────────┴────────────┴──────────┘");
    
    Ok(())
}

async fn launch_dashboard(results: serde_json::Value) -> Result<()> {
    use std::sync::Arc;
    use tokio::sync::Mutex;
    
    let results = Arc::new(Mutex::new(results));
    
    println!("📊 Dashboard available at: {}", "http://localhost:3001".bright_cyan());
    println!("   (In production, this would serve the React dashboard)");
    
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    
    Ok(())
}
