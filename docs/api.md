# API Reference

## Smart Contract API

### CognitiveRegistry

#### registerCognitiveNode
Register as a cognitive node with stake.

```solidity
function registerCognitiveNode(string memory nodeId) external payable
```

**Parameters:**
- `nodeId`: Unique identifier untuk node
- `msg.value`: Stake amount (minimum 100 MATIC)

**Events:**
- `NodeRegistered(address indexed validator, string nodeId)`

#### createReasoningTask
Create a new reasoning task.

```solidity
function createReasoningTask(
    string memory taskType,
    bytes32 dataHash,
    uint256 deadline
) external payable returns (uint256)
```

**Parameters:**
- `taskType`: Task type ("market_prediction", "anomaly_detection", etc)
- `dataHash`: Hash of task data
- `deadline`: Unix timestamp deadline
- `msg.value`: Reward amount

**Returns:** Task ID

**Events:**
- `TaskCreated(uint256 indexed taskId, string taskType, uint256 reward)`

#### assignTask
Assign task to your node.

```solidity
function assignTask(uint256 taskId) external
```

**Events:**
- `TaskAssigned(uint256 indexed taskId, address indexed node)`

#### submitTaskResult
Submit hasil reasoning task.

```solidity
function submitTaskResult(uint256 taskId, bytes32 resultHash) external
```

**Events:**
- `TaskCompleted(uint256 indexed taskId, address indexed node, uint256 reward)`
- `ReputationUpdated(address indexed node, uint256 newScore)`

### ProofOfReasoning

#### submitProof
Submit proof of reasoning.

```solidity
function submitProof(
    bytes32 inputHash,
    bytes32 outputHash,
    uint256 computationCost
) external returns (bytes32)
```

**Returns:** Proof ID

**Events:**
- `ProofSubmitted(bytes32 indexed proofId, address indexed prover)`

#### validateProof
Validate proof dari node lain.

```solidity
function validateProof(bytes32 proofId, bool approved) external
```

**Events:**
- `ProofValidated(bytes32 indexed proofId, address indexed validator, bool approved)`
- `ProofVerified(bytes32 indexed proofId, uint256 reward)` (jika consensus tercapai)

## Rust API

### Cognitive Engine

#### TaskProcessor

```rust
pub struct TaskProcessor {
    market_predictor: MarketPredictor,
    anomaly_detector: AnomalyDetector,
    risk_scorer: RiskScorer,
}

impl TaskProcessor {
    pub fn new() -> Self
    
    pub async fn process(&self, task: &ReasoningTask) -> Result<ReasoningResult>
}
```

#### MarketPredictor

```rust
pub struct MarketPredictor;

impl MarketPredictor {
    pub fn new() -> Self
    
    pub async fn predict(&self, data: &Value) -> Result<ReasoningResult>
}
```

**Input Format:**
```json
{
    "prices": [1.2, 1.3, 1.25, 1.4, 1.35]
}
```

**Output Format:**
```json
{
    "predicted_price": 1.35,
    "confidence": 0.85,
    "trend": "bullish"
}
```

#### AnomalyDetector

```rust
pub struct AnomalyDetector;

impl AnomalyDetector {
    pub fn new() -> Self
    
    pub async fn detect(&self, data: &Value) -> Result<ReasoningResult>
}
```

**Input Format:**
```json
{
    "transactions": [
        {"value": 100.0, "gas": 21000.0},
        {"value": 5000.0, "gas": 800000.0}
    ]
}
```

**Output Format:**
```json
{
    "anomalies_detected": 1,
    "anomaly_indices": [1],
    "severity": "high"
}
```

#### RiskScorer

```rust
pub struct RiskScorer;

impl RiskScorer {
    pub fn new() -> Self
    
    pub async fn score(&self, data: &Value) -> Result<ReasoningResult>
}
```

**Input Format:**
```json
{
    "contract_address": "0x...",
    "code_complexity": 0.7,
    "audited": false,
    "tx_volume": 50.0
}
```

**Output Format:**
```json
{
    "contract": "0x...",
    "risk_score": 0.75,
    "risk_level": "high",
    "factors": {
        "code_complexity": 0.7,
        "audited": false,
        "tx_volume": 50.0
    }
}
```

### Validator Plugin

#### ValidatorPlugin

```rust
pub struct ValidatorPlugin {
    provider: Arc<Provider<Http>>,
    wallet: LocalWallet,
    por_validator: ProofOfReasoningValidator,
    node_address: Address,
}

impl ValidatorPlugin {
    pub async fn new() -> Result<Self>
    
    pub async fn run(&self) -> Result<()>
}
```

### PoR Consensus

#### ConsensusEngine

```rust
pub struct ConsensusEngine {
    proofs: HashMap<String, ProofOfReasoning>,
    validators: Vec<String>,
    required_confirmations: u32,
}

impl ConsensusEngine {
    pub fn new(required_confirmations: u32) -> Self
    
    pub fn add_validator(&mut self, validator: String)
    
    pub fn submit_proof(&mut self, proof: ProofOfReasoning) -> Result<()>
    
    pub fn validate_proof(&mut self, proof_id: &str, validator: &str) -> Result<bool>
    
    pub fn get_verified_proofs(&self) -> Vec<&ProofOfReasoning>
    
    pub fn calculate_rewards(&self, proof_id: &str) -> Result<HashMap<String, u64>>
}
```

## Types

### ReasoningTask

```rust
pub struct ReasoningTask {
    pub task_id: u64,
    pub task_type: String,
    pub data: Value,
    pub requester: String,
    pub reward: u64,
    pub deadline: u64,
}
```

### ReasoningResult

```rust
pub struct ReasoningResult {
    pub prediction: Value,
    pub confidence_score: f64,
    pub computation_time_ms: u64,
}
```

### ProofOfReasoning

```rust
pub struct ProofOfReasoning {
    pub input_hash: String,
    pub output_hash: String,
    pub prover: String,
    pub timestamp: u64,
    pub verified: bool,
    pub confirmations: u32,
}
```

## Events

### NodeRegistered
```solidity
event NodeRegistered(address indexed validator, string nodeId)
```

### TaskCreated
```solidity
event TaskCreated(uint256 indexed taskId, string taskType, uint256 reward)
```

### TaskAssigned
```solidity
event TaskAssigned(uint256 indexed taskId, address indexed node)
```

### TaskCompleted
```solidity
event TaskCompleted(uint256 indexed taskId, address indexed node, uint256 reward)
```

### ProofSubmitted
```solidity
event ProofSubmitted(bytes32 indexed proofId, address indexed prover)
```

### ProofValidated
```solidity
event ProofValidated(bytes32 indexed proofId, address indexed validator, bool approved)
```

### ProofVerified
```solidity
event ProofVerified(bytes32 indexed proofId, uint256 reward)
```
