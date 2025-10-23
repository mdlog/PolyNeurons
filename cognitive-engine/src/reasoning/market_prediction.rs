use anyhow::Result;
use serde_json::Value;
use shared::types::ReasoningResult;
use tracing::info;

pub struct MarketPredictor;

impl MarketPredictor {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn predict(&self, data: &Value) -> Result<ReasoningResult> {
        info!("🔮 Running market prediction analysis...");
        
        // Ekstrak data historis
        let prices = data["prices"].as_array()
            .ok_or_else(|| anyhow::anyhow!("Invalid price data"))?;
        
        // Simple moving average prediction
        let prediction = self.calculate_sma(prices, 7)?;
        
        let confidence = self.calculate_confidence(prices)?;
        
        Ok(ReasoningResult {
            prediction: serde_json::json!({
                "predicted_price": prediction,
                "confidence": confidence,
                "trend": if prediction > 0.0 { "bullish" } else { "bearish" }
            }),
            confidence_score: confidence,
            computation_time_ms: 150,
        })
    }
    
    fn calculate_sma(&self, prices: &[Value], period: usize) -> Result<f64> {
        let recent_prices: Vec<f64> = prices
            .iter()
            .rev()
            .take(period)
            .filter_map(|v| v.as_f64())
            .collect();
        
        if recent_prices.is_empty() {
            return Ok(0.0);
        }
        
        let sum: f64 = recent_prices.iter().sum();
        Ok(sum / recent_prices.len() as f64)
    }
    
    fn calculate_confidence(&self, prices: &[Value]) -> Result<f64> {
        // Simplified confidence based on data quality
        let valid_count = prices.iter().filter(|v| v.is_number()).count();
        Ok((valid_count as f64 / prices.len() as f64) * 0.85)
    }
}
