use anyhow::Result;
use shared::types::{ReasoningTask, ReasoningResult};

use crate::reasoning::{
    market_prediction::MarketPredictor,
    anomaly_detection::AnomalyDetector,
    risk_scoring::RiskScorer,
};

pub struct TaskProcessor {
    market_predictor: MarketPredictor,
    anomaly_detector: AnomalyDetector,
    risk_scorer: RiskScorer,
}

impl TaskProcessor {
    pub fn new() -> Self {
        Self {
            market_predictor: MarketPredictor::new(),
            anomaly_detector: AnomalyDetector::new(),
            risk_scorer: RiskScorer::new(),
        }
    }
    
    pub async fn process(&self, task: &ReasoningTask) -> Result<ReasoningResult> {
        match task.task_type.as_str() {
            "market_prediction" => {
                self.market_predictor.predict(&task.data).await
            }
            "anomaly_detection" => {
                self.anomaly_detector.detect(&task.data).await
            }
            "risk_scoring" => {
                self.risk_scorer.score(&task.data).await
            }
            _ => {
                anyhow::bail!("Unknown task type: {}", task.task_type)
            }
        }
    }
}
