use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct MovementClient {
    rpc_url: String,
    client: reqwest::Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionPayload {
    #[serde(rename = "type")]
    pub tx_type: String,
    pub function: String,
    pub type_arguments: Vec<String>,
    pub arguments: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedTransaction {
    pub sender: String,
    pub sequence_number: String,
    pub max_gas_amount: String,
    pub gas_unit_price: String,
    pub expiration_timestamp_secs: String,
    pub payload: TransactionPayload,
    pub signature: TransactionSignature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionSignature {
    #[serde(rename = "type")]
    pub sig_type: String,
    pub public_key: String,
    pub signature: String,
}

#[derive(Debug, Deserialize)]
pub struct TransactionResponse {
    pub hash: String,
    pub sender: String,
    pub sequence_number: String,
    pub success: bool,
    pub gas_used: String,
    pub vm_status: String,
}

#[derive(Debug, Deserialize)]
pub struct AccountResource {
    #[serde(rename = "type")]
    pub resource_type: String,
    pub data: Value,
}

impl MovementClient {
    pub fn new(rpc_url: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");
        
        Self { rpc_url, client }
    }

    pub fn testnet() -> Self {
        Self::new("https://aptos.testnet.porto.movementlabs.xyz/v1".to_string())
    }

    pub fn mainnet() -> Self {
        Self::new("https://mainnet.movementnetwork.xyz/v1".to_string())
    }

    pub fn local() -> Self {
        Self::new("http://localhost:8545".to_string())
    }

    /// Get account info
    pub async fn get_account(&self, address: &str) -> Result<Value> {
        let url = format!("{}/accounts/{}", self.rpc_url, address);
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to send request")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to get account: {}", response.status());
        }

        let account = response.json().await?;
        Ok(account)
    }

    /// Get account resources
    pub async fn get_account_resources(&self, address: &str) -> Result<Vec<AccountResource>> {
        let url = format!("{}/accounts/{}/resources", self.rpc_url, address);
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to send request")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to get resources: {}", response.status());
        }

        let resources = response.json().await?;
        Ok(resources)
    }

    /// Submit a transaction
    pub async fn submit_transaction(&self, signed_tx: SignedTransaction) -> Result<String> {
        let url = format!("{}/transactions", self.rpc_url);
        let response = self.client
            .post(&url)
            .json(&signed_tx)
            .send()
            .await
            .context("Failed to submit transaction")?;

        if !response.status().is_success() {
            let error_body = response.text().await?;
            anyhow::bail!("Failed to submit transaction: {}", error_body);
        }

        let tx_response: TransactionResponse = response.json().await?;
        Ok(tx_response.hash)
    }

    /// Wait for transaction to be confirmed
    pub async fn wait_for_transaction(&self, tx_hash: &str) -> Result<TransactionResponse> {
        let url = format!("{}/transactions/by_hash/{}", self.rpc_url, tx_hash);
        
        for _ in 0..30 {
            tokio::time::sleep(Duration::from_secs(2)).await;
            
            let response = self.client
                .get(&url)
                .send()
                .await;

            if let Ok(resp) = response {
                if resp.status().is_success() {
                    let tx: TransactionResponse = resp.json().await?;
                    return Ok(tx);
                }
            }
        }

        anyhow::bail!("Transaction not found after 60 seconds")
    }

    /// Simulate a transaction
    pub async fn simulate_transaction(&self, signed_tx: SignedTransaction) -> Result<Value> {
        let url = format!("{}/transactions/simulate", self.rpc_url);
        let response = self.client
            .post(&url)
            .json(&signed_tx)
            .send()
            .await
            .context("Failed to simulate transaction")?;

        if !response.status().is_success() {
            let error_body = response.text().await?;
            anyhow::bail!("Simulation failed: {}", error_body);
        }

        let result = response.json().await?;
        Ok(result)
    }

    /// Get transaction by hash
    pub async fn get_transaction(&self, tx_hash: &str) -> Result<TransactionResponse> {
        let url = format!("{}/transactions/by_hash/{}", self.rpc_url, tx_hash);
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to get transaction")?;

        if !response.status().is_success() {
            anyhow::bail!("Transaction not found: {}", response.status());
        }

        let tx = response.json().await?;
        Ok(tx)
    }

    /// Estimate gas for a transaction
    pub async fn estimate_gas(&self, signed_tx: SignedTransaction) -> Result<u64> {
        let simulation = self.simulate_transaction(signed_tx).await?;
        
        let gas_used = simulation[0]["gas_used"]
            .as_str()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        Ok(gas_used)
    }

    /// Get chain ID
    pub async fn get_chain_id(&self) -> Result<u64> {
        let url = format!("{}/", self.rpc_url);
        let response = self.client
            .get(&url)
            .send()
            .await
            .context("Failed to get chain info")?;

        let info: Value = response.json().await?;
        let chain_id = info["chain_id"]
            .as_u64()
            .context("Missing chain_id")?;

        Ok(chain_id)
    }

    /// Check if account exists
    pub async fn account_exists(&self, address: &str) -> Result<bool> {
        match self.get_account(address).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Get account sequence number
    pub async fn get_sequence_number(&self, address: &str) -> Result<u64> {
        let account = self.get_account(address).await?;
        let seq = account["sequence_number"]
            .as_str()
            .and_then(|s| s.parse().ok())
            .context("Invalid sequence number")?;
        Ok(seq)
    }

    /// Health check
    pub async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/", self.rpc_url);
        let response = self.client
            .get(&url)
            .timeout(Duration::from_secs(5))
            .send()
            .await;

        match response {
            Ok(resp) => Ok(resp.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    pub fn rpc_url(&self) -> &str {
        &self.rpc_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_testnet_connection() {
        let client = MovementClient::testnet();
        let health = client.health_check().await;
        assert!(health.is_ok());
    }

    #[tokio::test]
    async fn test_get_chain_id() {
        let client = MovementClient::testnet();
        let chain_id = client.get_chain_id().await;
        assert!(chain_id.is_ok());
        println!("Movement Testnet Chain ID: {}", chain_id.unwrap());
    }
}
