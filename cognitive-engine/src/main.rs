use anyhow::Result;
use tracing::info;
use tracing_subscriber;

mod engine;
mod tasks;
mod reasoning;

use engine::CognitiveEngine;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    info!("🧠 Starting PolyNeurons Cognitive Engine...");
    
    let engine = CognitiveEngine::new().await?;
    
    info!("✅ Cognitive Engine initialized");
    info!("📡 Listening for reasoning tasks...");
    
    engine.run().await?;
    
    Ok(())
}
