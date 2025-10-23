use ethers::prelude::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Viewing Reasoning Task...");
    println!("");
    
    let rpc_url = std::env::var("AMOY_RPC_URL")
        .or_else(|_| std::env::var("POLYGON_RPC_URL"))?;
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let provider = Arc::new(provider);
    
    let registry_address: Address = std::env::var("REGISTRY_ADDRESS")?.parse()?;
    
    let abi = r#"[{
        "type": "function",
        "name": "reasoningTasks",
        "inputs": [{"name": "", "type": "uint256"}],
        "outputs": [
            {"name": "taskId", "type": "uint256"},
            {"name": "taskType", "type": "string"},
            {"name": "dataHash", "type": "bytes32"},
            {"name": "requester", "type": "address"},
            {"name": "reward", "type": "uint256"},
            {"name": "deadline", "type": "uint256"},
            {"name": "completed", "type": "bool"},
            {"name": "assignedNode", "type": "address"}
        ],
        "stateMutability": "view"
    }]"#;
    
    let contract = Contract::new(
        registry_address,
        serde_json::from_str::<ethers::abi::Abi>(abi)?,
        provider,
    );
    
    // Get task ID from args or default to 1
    // Note: When run via cargo, args start from index 0 as the binary name
    let args: Vec<String> = std::env::args().collect();
    let task_id = if args.len() > 1 {
        args[1].parse::<u64>().unwrap_or(1)
    } else {
        1
    };
    
    println!("🔍 Fetching Task #{}...", task_id);
    println!("");
    
    type TaskData = (U256, String, [u8; 32], Address, U256, U256, bool, Address);
    
    let task: TaskData = contract
        .method::<_, TaskData>("reasoningTasks", U256::from(task_id))?
        .call()
        .await?;
    
    let (task_id, task_type, data_hash, requester, reward, deadline, completed, assigned_node) = task;
    
    println!("📋 Task Details:");
    println!("   ID:           {}", task_id);
    println!("   Type:         {}", task_type);
    println!("   Data Hash:    0x{}", hex::encode(data_hash));
    println!("   Requester:    {:?}", requester);
    println!("   Reward:       {} MATIC", ethers::utils::format_ether(reward));
    println!("   Deadline:     {} (Unix timestamp)", deadline);
    println!("   Completed:    {}", if completed { "✅ Yes" } else { "⏳ No" });
    
    if assigned_node != Address::zero() {
        println!("   Assigned To:  {:?}", assigned_node);
    } else {
        println!("   Assigned To:  🔓 Unassigned (available)");
    }
    
    // Convert deadline to human readable
    if let Ok(deadline_secs) = deadline.try_into() {
        use std::time::{SystemTime, UNIX_EPOCH, Duration};
        let deadline_time = UNIX_EPOCH + Duration::from_secs(deadline_secs);
        let now = SystemTime::now();
        
        if let Ok(duration) = deadline_time.duration_since(now) {
            let hours = duration.as_secs() / 3600;
            let minutes = (duration.as_secs() % 3600) / 60;
            println!("   Time Left:    {}h {}m", hours, minutes);
        } else {
            println!("   Time Left:    ⚠️  Expired");
        }
    }
    
    println!("");
    println!("🔗 View on Explorer:");
    println!("   https://amoy.polygonscan.com/address/{:?}", registry_address);
    
    Ok(())
}
