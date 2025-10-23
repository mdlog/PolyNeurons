use ethers::prelude::*;
use std::sync::Arc;
use sha2::{Sha256, Digest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📋 Submitting Cheap Reasoning Task...");
    println!("💡 This version uses minimal reward for testing");
    println!("");
    
    let rpc_url = std::env::var("AMOY_RPC_URL")
        .or_else(|_| std::env::var("POLYGON_RPC_URL"))?;
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let provider = Arc::new(provider);
    
    let private_key = std::env::var("PRIVATE_KEY")?;
    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(80002u64);
    
    let client = SignerMiddleware::new(provider.clone(), wallet);
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
    
    let deadline = chrono::Utc::now().timestamp() as u64 + 3600;
    
    // Very small reward for testing - only 0.001 MATIC!
    let reward = ethers::utils::parse_ether("0.001")?;
    
    println!("💰 Reward: 0.001 MATIC (~$0.0008 USD)");
    println!("⛽ Estimating gas...");
    
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
    
    // Estimate gas first
    match tx.estimate_gas().await {
        Ok(gas_estimate) => {
            println!("   Estimated gas: {} units", gas_estimate);
            
            // Get current gas price
            let gas_price = provider.get_gas_price().await?;
            let gas_cost = gas_estimate * gas_price;
            let gas_cost_matic = ethers::utils::format_ether(gas_cost);
            
            println!("   Gas price: {} gwei", ethers::utils::format_units(gas_price, "gwei")?);
            println!("   Estimated gas cost: {} MATIC", gas_cost_matic);
            println!("");
            println!("📊 Total Cost Breakdown:");
            println!("   Reward:   0.001 MATIC");
            println!("   Gas fee:  {} MATIC", gas_cost_matic);
            println!("   ─────────────────────");
            
            let total = ethers::utils::parse_ether("0.001")? + gas_cost;
            println!("   TOTAL:    {} MATIC", ethers::utils::format_ether(total));
            println!("");
        }
        Err(e) => {
            println!("⚠️  Could not estimate gas: {}", e);
        }
    }
    
    println!("🚀 Sending transaction...");
    let pending_tx = tx.send().await?;
    let receipt = pending_tx.await?;
    
    println!("✅ Task submitted!");
    println!("   Tx Hash: {:?}", receipt.as_ref().unwrap().transaction_hash);
    
    if let Some(receipt) = receipt.as_ref() {
        if let Some(gas_used) = receipt.gas_used {
            println!("   ⛽ Actual Gas Used: {}", gas_used);
            
            if let Some(effective_gas_price) = receipt.effective_gas_price {
                let actual_cost = gas_used * effective_gas_price;
                println!("   💸 Actual Gas Cost: {} MATIC", ethers::utils::format_ether(actual_cost));
            }
        }
    }
    
    Ok(())
}
