use anyhow::Result;
use serde_json::Value;
use shared::types::ReasoningResult;
use tracing::info;

pub struct AnomalyDetector;

impl AnomalyDetector {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn detect(&self, data: &Value) -> Result<ReasoningResult> {
        info!("🔍 Running anomaly detection...");
        
        let transactions = data["transactions"].as_array()
            .ok_or_else(|| anyhow::anyhow!("Invalid transaction data"))?;
        
        let mut anomalies = Vec::new();
        let mut anomaly_scores = Vec::new();
        
        for (idx, tx) in transactions.iter().enumerate() {
            let score = self.calculate_anomaly_score(tx)?;
            
            if score > 0.7 {
                anomalies.push(idx);
                anomaly_scores.push(score);
            }
        }
        
        let avg_confidence = if !anomaly_scores.is_empty() {
            anomaly_scores.iter().sum::<f64>() / anomaly_scores.len() as f64
        } else {
            0.95
        };
        
        Ok(ReasoningResult {
            prediction: serde_json::json!({
                "anomalies_detected": anomalies.len(),
                "anomaly_indices": anomalies,
                "severity": if anomalies.len() > 5 { "high" } else { "low" }
            }),
            confidence_score: avg_confidence,
            computation_time_ms: 200,
        })
    }
    
    fn calculate_anomaly_score(&self, tx: &Value) -> Result<f64> {
        let value = tx["value"].as_f64().unwrap_or(0.0);
        let gas = tx["gas"].as_f64().unwrap_or(0.0);
        
        // Simple heuristic: unusual value or gas
        let value_score = if value > 1000.0 { 0.8 } else { 0.2 };
        let gas_score = if gas > 500000.0 { 0.7 } else { 0.1 };
        
        Ok((value_score + gas_score) / 2.0)
    }
}
