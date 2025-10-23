use ethers::prelude::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔌 Registering Cognitive Node...");
    
    let rpc_url = std::env::var("AMOY_RPC_URL")
        .or_else(|_| std::env::var("POLYGON_RPC_URL"))?;
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let provider = Arc::new(provider);
    
    let private_key = std::env::var("PRIVATE_KEY")?;
    let wallet: LocalWallet = private_key.parse()?;
    let wallet = wallet.with_chain_id(80002u64); // Polygon Amoy testnet
    
    let client = SignerMiddleware::new(provider, wallet);
    
    let registry_address: Address = std::env::var("REGISTRY_ADDRESS")?.parse()?;
    
    // ABI for registerCognitiveNode
    let abi = r#"[{
        "type": "function",
        "name": "registerCognitiveNode",
        "inputs": [{"name": "nodeId", "type": "string"}],
        "outputs": [],
        "stateMutability": "payable"
    }]"#;
    
    let contract = Contract::new(
        registry_address,
        serde_json::from_str::<ethers::abi::Abi>(abi)?,
        Arc::new(client),
    );
    
    let node_id = format!("polyneuron-{}", chrono::Utc::now().timestamp());
    let stake_amount = ethers::utils::parse_ether("0.1")?; // 0.1 MATIC (testnet)
    
    println!("📝 Node ID: {}", node_id);
    println!("💰 Stake: 0.1 MATIC (testnet amount)");
    
    let tx = contract
        .method::<_, ()>("registerCognitiveNode", node_id)?
        .value(stake_amount);
    
    let pending_tx = tx.send().await?;
    let receipt = pending_tx.await?;
    
    println!("✅ Node registered!");
    println!("   Tx Hash: {:?}", receipt.unwrap().transaction_hash);
    
    Ok(())
}
