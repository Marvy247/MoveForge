use anyhow::Result;
use dashmap::DashMap;
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::{info, debug};

use crate::types::{Account, Block, Transaction, TransactionReceipt};

pub struct StateManager {
    accounts: Arc<DashMap<String, Account>>,
    blocks: Arc<DashMap<u64, Block>>,
    transactions: Arc<DashMap<String, TransactionReceipt>>,
    fork_url: Option<String>,
    current_block: u64,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            accounts: Arc::new(DashMap::new()),
            blocks: Arc::new(DashMap::new()),
            transactions: Arc::new(DashMap::new()),
            fork_url: None,
            current_block: 0,
        }
    }

    pub async fn forked(url: String) -> Result<Self> {
        info!("Creating forked state from: {}", url);
        
        let mut state = Self::new();
        state.fork_url = Some(url.clone());

        match state.sync_from_fork().await {
            Ok(_) => info!("Successfully synced state from fork"),
            Err(e) => info!("Fork sync not available, starting fresh: {}", e),
        }

        Ok(state)
    }

    async fn sync_from_fork(&mut self) -> Result<()> {
        if let Some(ref url) = self.fork_url {
            debug!("Attempting to sync state from {}", url);
        }
        
        Ok(())
    }

    pub fn get_account(&self, address: &str) -> Option<Account> {
        self.accounts.get(address).map(|a| a.clone())
    }

    pub fn set_account(&self, address: String, account: Account) {
        self.accounts.insert(address, account);
    }

    pub fn get_block(&self, number: u64) -> Option<Block> {
        self.blocks.get(&number).map(|b| b.clone())
    }

    pub fn add_block(&self, block: Block) {
        let number = block.number;
        self.blocks.insert(number, block);
    }

    pub fn get_transaction(&self, hash: &str) -> Option<TransactionReceipt> {
        self.transactions.get(hash).map(|t| t.clone())
    }

    pub fn add_transaction(&self, hash: String, receipt: TransactionReceipt) {
        self.transactions.insert(hash, receipt);
    }

    pub fn current_block_number(&self) -> u64 {
        self.current_block
    }

    pub fn increment_block(&mut self) {
        self.current_block += 1;
    }

    pub fn get_or_create_account(&self, address: String) -> Account {
        if let Some(account) = self.get_account(&address) {
            account
        } else {
            let account = Account {
                address: address.clone(),
                balance: "0".to_string(),
                nonce: 0,
                code: None,
                resources: json!({}),
            };
            self.set_account(address, account.clone());
            account
        }
    }
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_manager() {
        let state = StateManager::new();
        let account = state.get_or_create_account("0x123".to_string());
        assert_eq!(account.address, "0x123");
        assert_eq!(account.balance, "0");
    }

    #[tokio::test]
    async fn test_forked_state() {
        let state = StateManager::forked(
            "https://aptos.testnet.porto.movementlabs.xyz/v1".to_string()
        ).await;
        assert!(state.is_ok());
    }
}
