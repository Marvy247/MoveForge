use anyhow::Result;
use rayon::prelude::*;
use serde_json::{json, Value};
use std::time::Instant;
use tracing::{info, debug};

pub struct Simulator {
    parallel: bool,
}

impl Simulator {
    pub fn new(parallel: bool) -> Self {
        Self { parallel }
    }

    pub async fn simulate_batch(&self, transactions: Value) -> Result<Value> {
        let txs = transactions.as_array()
            .ok_or_else(|| anyhow::anyhow!("Expected array of transactions"))?;

        info!("Simulating {} transactions (parallel: {})", txs.len(), self.parallel);

        let results = if self.parallel {
            self.simulate_parallel(txs).await?
        } else {
            self.simulate_sequential(txs).await?
        };

        Ok(json!(results))
    }

    async fn simulate_parallel(&self, txs: &[Value]) -> Result<Vec<Value>> {
        let results: Vec<Value> = txs.par_iter()
            .enumerate()
            .map(|(i, tx)| {
                let start = Instant::now();
                
                let result = self.simulate_single(tx, i);
                
                let duration = start.elapsed();
                
                let mut res = result.unwrap_or_else(|_| json!({
                    "id": format!("tx_{}", i),
                    "status": "failed",
                }));
                
                res["execution_time_ms"] = json!(duration.as_millis() as f64);
                res
            })
            .collect();

        Ok(results)
    }

    async fn simulate_sequential(&self, txs: &[Value]) -> Result<Vec<Value>> {
        let mut results = Vec::new();

        for (i, tx) in txs.iter().enumerate() {
            let start = Instant::now();
            
            let mut result = self.simulate_single(tx, i)?;
            
            let duration = start.elapsed();
            result["execution_time_ms"] = json!(duration.as_millis() as f64);
            
            results.push(result);
        }

        Ok(results)
    }

    fn simulate_single(&self, tx: &Value, index: usize) -> Result<Value> {
        let tx_id = tx["id"].as_str()
            .unwrap_or(&format!("tx_{}", index))
            .to_string();
        
        let tx_type = tx["type"].as_str().unwrap_or("transfer");
        
        let gas_used = match tx_type {
            "transfer" => 21000,
            "swap" => 85000 + index * 100,
            "mint" => 120000 + index * 150,
            _ => 50000,
        };

        std::thread::sleep(std::time::Duration::from_micros(100));

        let has_conflict = index % 7 == 0;
        let status = if has_conflict { "failed" } else { "success" };

        Ok(json!({
            "id": tx_id,
            "type": tx_type,
            "status": status,
            "gas_used": gas_used,
            "resource_changes": self.generate_resource_changes(tx_type),
            "conflicts": if has_conflict { vec!["resource_0x123"] } else { vec![] },
        }))
    }

    fn generate_resource_changes(&self, tx_type: &str) -> Vec<Value> {
        match tx_type {
            "transfer" => vec![
                json!({
                    "address": "0xsender",
                    "resource": "Coin<MOVE>",
                    "change": "decrease"
                }),
                json!({
                    "address": "0xrecipient",
                    "resource": "Coin<MOVE>",
                    "change": "increase"
                }),
            ],
            "swap" => vec![
                json!({
                    "address": "0xpool",
                    "resource": "LiquidityPool",
                    "change": "update"
                }),
            ],
            "mint" => vec![
                json!({
                    "address": "0xminter",
                    "resource": "NFT",
                    "change": "create"
                }),
            ],
            _ => vec![],
        }
    }

    pub async fn analyze_conflicts(&self, results: &Value) -> Result<Value> {
        let txs = results.as_array()
            .ok_or_else(|| anyhow::anyhow!("Expected array"))?;

        let mut conflicts = Vec::new();
        let mut resource_map: std::collections::HashMap<String, Vec<usize>> = 
            std::collections::HashMap::new();

        for (i, tx) in txs.iter().enumerate() {
            if let Some(changes) = tx["resource_changes"].as_array() {
                for change in changes {
                    if let Some(resource) = change["resource"].as_str() {
                        resource_map.entry(resource.to_string())
                            .or_insert_with(Vec::new)
                            .push(i);
                    }
                }
            }
        }

        for (resource, tx_indices) in resource_map {
            if tx_indices.len() > 1 {
                conflicts.push(json!({
                    "resource": resource,
                    "conflicting_txs": tx_indices,
                    "severity": "high",
                }));
            }
        }

        Ok(json!({
            "total_conflicts": conflicts.len(),
            "conflicts": conflicts,
            "recommendation": if conflicts.is_empty() {
                "No conflicts detected. Transactions can execute in parallel safely."
            } else {
                "Conflicts detected. Consider retry logic or transaction reordering."
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_simulator() {
        let sim = Simulator::new(true);
        let txs = json!([
            {"id": "tx_1", "type": "transfer"},
            {"id": "tx_2", "type": "swap"},
        ]);
        
        let results = sim.simulate_batch(txs).await;
        assert!(results.is_ok());
    }

    #[tokio::test]
    async fn test_conflict_analysis() {
        let sim = Simulator::new(true);
        let results = json!([
            {
                "id": "tx_1",
                "resource_changes": [
                    {"resource": "Coin<MOVE>"}
                ]
            },
            {
                "id": "tx_2",
                "resource_changes": [
                    {"resource": "Coin<MOVE>"}
                ]
            }
        ]);
        
        let analysis = sim.analyze_conflicts(&results).await;
        assert!(analysis.is_ok());
    }
}
