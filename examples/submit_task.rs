use ethers::prelude::*;
use std::sync::Arc;
use sha2::{Sha256, Digest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📋 Submitting Reasoning Task...");
    
    let rpc_url = std::env::var("AMOY_RPC_URL")
        .or_else(|_| std::env::var("POLYGON_RPC_URL"))?;
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let provider = Arc::new(provider);
    
    let private_key = std::env::var("PRIVATE_KEY")?;
    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(80002u64); // Polygon Amoy testnet
    
    let client = SignerMiddleware::new(provider, wallet);
    let registry_address: Address = std::env::var("REGISTRY_ADDRESS")?.parse()?;
    
    // Task data
    let task_data = serde_json::json!({
        "type": "market_prediction",
        "symbol": "MATIC/USD",
        "prices": [1.2, 1.3, 1.25, 1.4, 1.35]
    });
    
    // Hash data
    let mut hasher = Sha256::new();
    hasher.update(task_data.to_string().as_bytes());
    let hash_result = hasher.finalize();
    
    // Convert to [u8; 32] for bytes32
    let mut data_hash = [0u8; 32];
    data_hash.copy_from_slice(&hash_result);
    
    println!("🎯 Task Type: market_prediction");
    println!("📊 Data Hash: 0x{}", hex::encode(&data_hash));
    
    let abi = r#"[{
        "type": "function",
        "name": "createReasoningTask",
        "inputs": [
            {"name": "taskType", "type": "string"},
            {"name": "dataHash", "type": "bytes32"},
            {"name": "deadline", "type": "uint256"}
        ],
        "outputs": [{"name": "", "type": "uint256"}],
        "stateMutability": "payable"
    }]"#;
    
    let contract = Contract::new(
        registry_address,
        serde_json::from_str::<ethers::abi::Abi>(abi)?,
        Arc::new(client),
    );
    
    let deadline = chrono::Utc::now().timestamp() as u64 + 3600; // 1 hour
    let reward = ethers::utils::parse_ether("1")?; // 1 MATIC reward
    
    let tx = contract
        .method::<_, U256>(
            "createReasoningTask",
            (
                "market_prediction".to_string(),
                data_hash,
                U256::from(deadline),
            ),
        )?
        .value(reward);
    
    let pending_tx = tx.send().await?;
    let receipt = pending_tx.await?;
    
    println!("✅ Task submitted!");
    println!("   Tx Hash: {:?}", receipt.as_ref().unwrap().transaction_hash);
    println!("   💰 Reward: 1 MATIC");
    
    Ok(())
}
