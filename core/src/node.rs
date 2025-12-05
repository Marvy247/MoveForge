use anyhow::Result;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

use crate::state::StateManager;
use crate::rpc::RpcServer;

pub struct LocalNode {
    port: u16,
    fork_url: Option<String>,
    parallel: bool,
    state: Arc<RwLock<StateManager>>,
}

impl LocalNode {
    pub async fn new(port: u16, fork_url: Option<String>, parallel: bool) -> Result<Self> {
        let state = if let Some(ref url) = fork_url {
            StateManager::forked(url.clone()).await?
        } else {
            StateManager::new()
        };

        Ok(Self {
            port,
            fork_url,
            parallel,
            state: Arc::new(RwLock::new(state)),
        })
    }

    pub async fn run(&self) -> Result<()> {
        info!("Starting ParaForge node on port {}", self.port);
        
        if let Some(ref url) = self.fork_url {
            info!("Forked from: {}", url);
        }

        let rpc_server = RpcServer::new(
            self.port,
            self.state.clone(),
            self.parallel,
        );

        rpc_server.start().await?;

        Ok(())
    }

    pub async fn get_state(&self) -> Arc<RwLock<StateManager>> {
        self.state.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_node_creation() {
        let node = LocalNode::new(8545, None, true).await;
        assert!(node.is_ok());
    }

    #[tokio::test]
    async fn test_node_with_fork() {
        let node = LocalNode::new(
            8545,
            Some("https://aptos.testnet.porto.movementlabs.xyz/v1".to_string()),
            true
        ).await;
        assert!(node.is_ok());
    }
}
