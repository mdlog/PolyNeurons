use anyhow::Result;
use tracing::info;
use tracing_subscriber;

mod plugin;
mod por;

use plugin::ValidatorPlugin;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    info!("🔌 Starting PolyNeurons Validator Plugin...");
    
    let plugin = ValidatorPlugin::new().await?;
    
    info!("✅ Validator Plugin initialized");
    info!("🔗 Connected to Polygon network");
    
    plugin.run().await?;
    
    Ok(())
}
