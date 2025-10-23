use ethers::prelude::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Viewing Cognitive Node Status...");
    println!("");
    
    let rpc_url = std::env::var("AMOY_RPC_URL")
        .or_else(|_| std::env::var("POLYGON_RPC_URL"))?;
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let provider = Arc::new(provider);
    
    let registry_address: Address = std::env::var("REGISTRY_ADDRESS")?.parse()?;
    
    // Get node address from private key
    let private_key = std::env::var("PRIVATE_KEY")?;
    let wallet: LocalWallet = private_key.parse()?;
    let node_address = wallet.address();
    
    let abi = r#"[{
        "type": "function",
        "name": "cognitiveNodes",
        "inputs": [{"name": "", "type": "address"}],
        "outputs": [
            {"name": "validator", "type": "address"},
            {"name": "nodeId", "type": "string"},
            {"name": "stakedAmount", "type": "uint256"},
            {"name": "reputationScore", "type": "uint256"},
            {"name": "tasksCompleted", "type": "uint256"},
            {"name": "isActive", "type": "bool"},
            {"name": "registeredAt", "type": "uint256"}
        ],
        "stateMutability": "view"
    }]"#;
    
    let contract = Contract::new(
        registry_address,
        serde_json::from_str::<ethers::abi::Abi>(abi)?,
        provider,
    );
    
    type NodeData = (Address, String, U256, U256, U256, bool, U256);
    
    let node: NodeData = contract
        .method::<_, NodeData>("cognitiveNodes", node_address)?
        .call()
        .await?;
    
    let (validator, node_id, staked, reputation, tasks_completed, is_active, registered_at) = node;
    
    if validator == Address::zero() {
        println!("❌ Node not registered");
        println!("   Address: {:?}", node_address);
        println!("");
        println!("💡 Register with: make register-node");
        return Ok(());
    }
    
    println!("🔌 Cognitive Node Details:");
    println!("   Address:         {:?}", validator);
    println!("   Node ID:         {}", node_id);
    println!("   Staked:          {} MATIC", ethers::utils::format_ether(staked));
    println!("   Reputation:      {} points", reputation);
    println!("   Tasks Completed: {}", tasks_completed);
    println!("   Status:          {}", if is_active { "✅ Active" } else { "❌ Inactive" });
    
    // Convert timestamp to human readable
    if let Ok(timestamp) = registered_at.try_into() {
        use std::time::{SystemTime, UNIX_EPOCH, Duration};
        let registered_time = UNIX_EPOCH + Duration::from_secs(timestamp);
        let now = SystemTime::now();
        
        if let Ok(duration) = now.duration_since(registered_time) {
            let hours = duration.as_secs() / 3600;
            let days = hours / 24;
            
            if days > 0 {
                println!("   Registered:      {} days ago", days);
            } else {
                println!("   Registered:      {} hours ago", hours);
            }
        }
    }
    
    println!("");
    println!("🔗 View on Explorer:");
    println!("   https://amoy.polygonscan.com/address/{:?}", registry_address);
    
    Ok(())
}
