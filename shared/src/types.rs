use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningTask {
    pub task_id: u64,
    pub task_type: String,
    pub data: Value,
    pub requester: String,
    pub reward: u64,
    pub deadline: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningResult {
    pub prediction: Value,
    pub confidence_score: f64,
    pub computation_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveNode {
    pub validator_address: String,
    pub node_id: String,
    pub reputation_score: u64,
    pub tasks_completed: u64,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofOfReasoning {
    pub input_hash: String,
    pub output_hash: String,
    pub prover: String,
    pub timestamp: u64,
    pub verified: bool,
    pub confirmations: u32,
}
