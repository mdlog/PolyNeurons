# PolyNeurons Architecture

## Overview

PolyNeurons is a hybrid system that combines blockchain validation with AI reasoning tasks in a single infrastructure.

## Main Components

### 1. Cognitive Engine (Rust)
- **Location**: `cognitive-engine/`
- **Function**: Execute reasoning tasks
- **Tasks**:
  - Market Prediction: Price trend analysis
  - Anomaly Detection: Suspicious transaction detection
  - Risk Scoring: Contract risk assessment
  - Pattern Recognition: On-chain pattern identification

### 2. Validator Plugin (Rust)
- **Location**: `validator-plugin/`
- **Function**: Plugin for Polygon validators
- **Features**:
  - Dual-mode operation: Block validation + Reasoning tasks
  - Task assignment from smart contract
  - Proof of Reasoning submission
  - Peer proof validation

### 3. PoR Consensus (Rust)
- **Location**: `por-consensus/`
- **Function**: Consensus for reasoning results
- **Mechanism**:
  - Multi-validator verification
  - Reputation-based weighting
  - Reward distribution

### 4. Smart Contracts (Solidity)
- **Location**: `contracts/`
- **Contracts**:
  - `CognitiveRegistry.sol`: Node registration & task management
  - `ProofOfReasoning.sol`: Proof submission & validation

## Flow Diagram

```
┌─────────────────┐
│   Requester     │
│  (Submit Task)  │
└────────┬────────┘
         │
         ▼
┌─────────────────────────┐
│  CognitiveRegistry      │
│  Smart Contract         │
└────────┬────────────────┘
         │
         ▼
┌─────────────────────────┐
│  Cognitive Nodes        │
│  (Assign & Process)     │
└────────┬────────────────┘
         │
         ▼
┌─────────────────────────┐
│  Cognitive Engine       │
│  (Execute Reasoning)    │
└────────┬────────────────┘
         │
         ▼
┌─────────────────────────┐
│  Generate PoR           │
│  (Input/Output Hash)    │
└────────┬────────────────┘
         │
         ▼
┌─────────────────────────┐
│  Submit to Blockchain   │
│  (ProofOfReasoning)     │
└────────┬────────────────┘
         │
         ▼
┌─────────────────────────┐
│  Peer Validation        │
│  (Multi-validator)      │
└────────┬────────────────┘
         │
         ▼
┌─────────────────────────┐
│  Consensus Reached      │
│  (Distribute Rewards)   │
└─────────────────────────┘
```

## Proof of Reasoning (PoR)

### Concept
PoR is a consensus mechanism that validates reasoning results without re-executing the entire computation.

### Process
1. **Input Hashing**: Hash of task data
2. **Computation**: Execute reasoning task
3. **Output Hashing**: Hash of reasoning result
4. **Proof Submission**: Submit (input_hash, output_hash) to blockchain
5. **Peer Validation**: Other validators verify
6. **Consensus**: After N confirmations, proof is accepted
7. **Reward**: Distribute rewards to prover & validators

### Security
- Stake requirement prevents spam
- Multi-validator consensus prevents fraud
- Reputation system for quality control
- Slashing for invalid proofs

## Reward Economics

### Block Reward Split
- **80%**: Standard block validation
- **20%**: Cognitive task contribution

### Task Reward Distribution
- **70%**: Prover (who executes reasoning)
- **30%**: Validators (who verify proof)

### Reputation Impact
- Successful tasks: +10 reputation
- Failed validation: -20 reputation
- Reputation affects task assignment priority

## Scalability

### Horizontal Scaling
- Multiple cognitive nodes can operate in parallel
- Task distribution based on node capacity
- Automatic load balancing

### Vertical Scaling
- Optimized Rust implementation
- Async/await for concurrency
- Minimal memory footprint

## Integration with Polygon

### Bor Client
Validator plugin integrates with Bor (Polygon's validator client):
- Hook into block validation pipeline
- Parallel execution with block processing
- Minimal overhead (<5% CPU)

### Heimdall
Checkpoint submission for reasoning results:
- Periodic batching
- Ethereum mainnet finality
- Cross-chain verification
