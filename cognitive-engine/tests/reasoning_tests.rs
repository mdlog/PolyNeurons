#[cfg(test)]
mod tests {
    use cognitive_engine::reasoning::{
        market_prediction::MarketPredictor,
        anomaly_detection::AnomalyDetector,
        risk_scoring::RiskScorer,
    };
    use serde_json::json;

    #[tokio::test]
    async fn test_market_prediction() {
        let predictor = MarketPredictor::new();
        
        let data = json!({
            "prices": [1.2, 1.3, 1.25, 1.4, 1.35, 1.5, 1.45]
        });
        
        let result = predictor.predict(&data).await.unwrap();
        
        assert!(result.confidence_score > 0.0);
        assert!(result.confidence_score <= 1.0);
        assert!(result.prediction["predicted_price"].is_number());
    }

    #[tokio::test]
    async fn test_anomaly_detection() {
        let detector = AnomalyDetector::new();
        
        let data = json!({
            "transactions": [
                {"value": 100.0, "gas": 21000.0},
                {"value": 5000.0, "gas": 800000.0}, // Anomaly
                {"value": 50.0, "gas": 30000.0},
            ]
        });
        
        let result = detector.detect(&data).await.unwrap();
        
        assert!(result.prediction["anomalies_detected"].as_u64().unwrap() > 0);
    }

    #[tokio::test]
    async fn test_risk_scoring() {
        let scorer = RiskScorer::new();
        
        let data = json!({
            "contract_address": "0x1234567890123456789012345678901234567890",
            "code_complexity": 0.7,
            "audited": false,
            "tx_volume": 50.0
        });
        
        let result = scorer.score(&data).await.unwrap();
        
        let risk_score = result.prediction["risk_score"].as_f64().unwrap();
        assert!(risk_score >= 0.0 && risk_score <= 1.0);
        
        let risk_level = result.prediction["risk_level"].as_str().unwrap();
        assert!(["low", "medium", "high", "critical"].contains(&risk_level));
    }
}
