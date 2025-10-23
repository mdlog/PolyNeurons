use anyhow::Result;
use ethers::prelude::*;
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tracing::{info, warn};

use crate::tasks::TaskProcessor;
use shared::types::{ReasoningTask, ReasoningResult};

pub struct CognitiveEngine {
    _provider: Arc<Provider<Http>>,
    _wallet: LocalWallet,
    _registry_address: Address,
    task_processor: TaskProcessor,
}

impl CognitiveEngine {
    pub async fn new() -> Result<Self> {
        let rpc_url = std::env::var("POLYGON_RPC_URL")
            .unwrap_or_else(|_| "http://localhost:8545".to_string());
        
        let provider = Provider::<Http>::try_from(rpc_url)?;
        let provider = Arc::new(provider);
        
        let private_key = std::env::var("PRIVATE_KEY")
            .unwrap_or_else(|_| "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string());
        let wallet: LocalWallet = private_key.parse()?;
        
        let registry_address = std::env::var("REGISTRY_ADDRESS")
            .unwrap_or_else(|_| "0x0000000000000000000000000000000000000000".to_string())
            .parse()?;
        
        let task_processor = TaskProcessor::new();
        
        Ok(Self {
            _provider: provider,
            _wallet: wallet,
            _registry_address: registry_address,
            task_processor,
        })
    }
    
    pub async fn run(&self) -> Result<()> {
        let mut ticker = interval(Duration::from_secs(10));
        
        loop {
            ticker.tick().await;
            
            if let Err(e) = self.process_pending_tasks().await {
                warn!("Error processing tasks: {}", e);
            }
        }
    }
    
    async fn process_pending_tasks(&self) -> Result<()> {
        info!("🔍 Checking for pending tasks...");
        
        let tasks = self.fetch_pending_tasks().await?;
        
        for task in tasks {
            info!("📋 Processing task: {} (type: {})", task.task_id, task.task_type);
            
            match self.task_processor.process(&task).await {
                Ok(result) => {
                    info!("✅ Task {} completed", task.task_id);
                    self.submit_result(task.task_id, result).await?;
                }
                Err(e) => {
                    warn!("❌ Task {} failed: {}", task.task_id, e);
                }
            }
        }
        
        Ok(())
    }
    
    async fn fetch_pending_tasks(&self) -> Result<Vec<ReasoningTask>> {
        // Simulasi - dalam implementasi nyata, query dari smart contract
        Ok(vec![])
    }
    
    async fn submit_result(&self, task_id: u64, _result: ReasoningResult) -> Result<()> {
        info!("📤 Submitting result for task {}", task_id);
        // Submit to smart contract
        Ok(())
    }
}
