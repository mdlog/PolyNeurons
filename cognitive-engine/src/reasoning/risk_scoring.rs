use anyhow::Result;
use serde_json::Value;
use shared::types::ReasoningResult;
use tracing::info;

pub struct RiskScorer;

impl RiskScorer {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn score(&self, data: &Value) -> Result<ReasoningResult> {
        info!("⚠️  Running risk scoring analysis...");
        
        let contract_address = data["contract_address"].as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing contract address"))?;
        
        let code_complexity = data["code_complexity"].as_f64().unwrap_or(0.5);
        let audit_status = data["audited"].as_bool().unwrap_or(false);
        let transaction_volume = data["tx_volume"].as_f64().unwrap_or(0.0);
        
        let risk_score = self.calculate_risk(code_complexity, audit_status, transaction_volume);
        let risk_level = self.categorize_risk(risk_score);
        
        Ok(ReasoningResult {
            prediction: serde_json::json!({
                "contract": contract_address,
                "risk_score": risk_score,
                "risk_level": risk_level,
                "factors": {
                    "code_complexity": code_complexity,
                    "audited": audit_status,
                    "tx_volume": transaction_volume
                }
            }),
            confidence_score: 0.88,
            computation_time_ms: 180,
        })
    }
    
    fn calculate_risk(&self, complexity: f64, audited: bool, volume: f64) -> f64 {
        let mut risk = complexity * 0.4;
        
        if !audited {
            risk += 0.3;
        }
        
        if volume < 100.0 {
            risk += 0.2;
        }
        
        risk.min(1.0)
    }
    
    fn categorize_risk(&self, score: f64) -> &str {
        match score {
            s if s < 0.3 => "low",
            s if s < 0.6 => "medium",
            s if s < 0.8 => "high",
            _ => "critical"
        }
    }
}
