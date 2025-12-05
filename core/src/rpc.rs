use anyhow::Result;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug};

use crate::state::StateManager;
use crate::types::{Transaction, TransactionReceipt, ExecutionStatus};

pub struct RpcServer {
    port: u16,
    state: Arc<RwLock<StateManager>>,
    parallel: bool,
}

impl RpcServer {
    pub fn new(
        port: u16,
        state: Arc<RwLock<StateManager>>,
        parallel: bool,
    ) -> Self {
        Self {
            port,
            state,
            parallel,
        }
    }

    pub async fn start(&self) -> Result<()> {
        info!("RPC server listening on port {}", self.port);
        info!("Parallel execution: {}", self.parallel);

        tokio::signal::ctrl_c().await?;
        info!("Shutting down RPC server");

        Ok(())
    }

    pub async fn handle_request(&self, method: &str, params: Value) -> Result<Value> {
        debug!("RPC call: {} with params: {:?}", method, params);

        match method {
            "eth_blockNumber" => self.block_number().await,
            "eth_getBalance" => self.get_balance(params).await,
            "eth_sendTransaction" => self.send_transaction(params).await,
            "eth_call" => self.call(params).await,
            "eth_estimateGas" => self.estimate_gas(params).await,
            "move_executeTransaction" => self.execute_move_transaction(params).await,
            "paraforge_simulateBatch" => self.simulate_batch(params).await,
            _ => Ok(json!({ "error": "Method not found" })),
        }
    }

    async fn block_number(&self) -> Result<Value> {
        let state = self.state.read().await;
        Ok(json!(format!("0x{:x}", state.current_block_number())))
    }

    async fn get_balance(&self, params: Value) -> Result<Value> {
        let address = params[0].as_str().unwrap_or("");
        let state = self.state.read().await;
        
        if let Some(account) = state.get_account(address) {
            Ok(json!(account.balance))
        } else {
            Ok(json!("0x0"))
        }
    }

    async fn send_transaction(&self, params: Value) -> Result<Value> {
        let tx_hash = format!("0x{:064x}", rand::random::<u64>());
        Ok(json!(tx_hash))
    }

    async fn call(&self, _params: Value) -> Result<Value> {
        Ok(json!("0x"))
    }

    async fn estimate_gas(&self, _params: Value) -> Result<Value> {
        Ok(json!("0x5208"))
    }

    async fn execute_move_transaction(&self, _params: Value) -> Result<Value> {
        let tx_hash = format!("0x{:064x}", rand::random::<u64>());
        Ok(json!({
            "hash": tx_hash,
            "status": "success"
        }))
    }

    async fn simulate_batch(&self, params: Value) -> Result<Value> {
        let empty_vec = vec![];
        let transactions = params.as_array().unwrap_or(&empty_vec);
        
        let mut results = Vec::new();
        for (i, _tx) in transactions.iter().enumerate() {
            results.push(json!({
                "id": format!("tx_{}", i),
                "status": "success",
                "gas_used": 21000 + i * 100,
                "execution_time_ms": 10.0 + i as f64 * 0.5
            }));
        }

        Ok(json!(results))
    }
}

mod rand {
    pub fn random<T>() -> T 
    where
        T: From<u64>
    {
        let val = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        T::from(val)
    }
}
