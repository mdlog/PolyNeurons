# PolyNeurons - On-chain Cognitive Nodes

> Blockchain that thinks while validating. Polygon nodes that execute reasoning tasks with Proof of Reasoning (PoR).

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)]()
[![Rust](https://img.shields.io/badge/rust-1.90%2B-orange)]()
[![Solidity](https://img.shields.io/badge/solidity-0.8.20-blue)]()
[![License](https://img.shields.io/badge/license-MIT-green)]()

## 🧠 Concept

**PolyNeurons** combines blockchain validation with AI reasoning in a single infrastructure:

- **Cognitive Nodes**: Validators that execute reasoning tasks
- **Proof of Reasoning (PoR)**: Consensus to validate AI results
- **Dual Rewards**: Block reward (80%) + reasoning reward (20%)

## ⚡ Quick Start

```bash
# 1. Install & Build
make install && make build

# 2. Deploy contracts (local)
npx hardhat node &
npm run deploy

# 3. Run services
make run-engine      # Terminal 1
make run-validator   # Terminal 2
```

📖 See [QUICKSTART.md](QUICKSTART.md) for complete guide.

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Polygon Network                       │
├─────────────────────────────────────────────────────────┤
│  Block Validation (80%)  │  Reasoning Tasks (20%)       │
│  ├─ Standard PoS         │  ├─ Market Prediction        │
│  ├─ Transaction verify   │  ├─ Anomaly Detection        │
│  └─ Consensus            │  ├─ Risk Scoring             │
│                          │  └─ Pattern Recognition       │
└─────────────────────────────────────────────────────────┘
         ▲                           ▲
         │                           │
    ┌────┴────┐              ┌──────┴──────┐
    │   Bor   │              │  Cognitive  │
    │Validator│◄────────────►│   Engine    │
    └─────────┘              └─────────────┘
                                   (Rust)
```

## 🎯 Cognitive Tasks

| Task | Description | Output |
|------|-------------|--------|
| **Market Prediction** | Price trend analysis with SMA | Predicted price, trend, confidence |
| **Anomaly Detection** | Suspicious transaction detection | Anomaly indices, severity |
| **Risk Scoring** | Smart contract risk assessment | Risk score (0-1), risk level |
| **Pattern Recognition** | On-chain pattern identification | Pattern type, frequency |

## 📊 Proof of Reasoning Flow

```rust
// 1. Node receives task
let task = fetch_task_from_contract().await?;

// 2. Execute reasoning
let result = cognitive_engine.process(&task).await?;

// 3. Generate proof
let proof = ProofOfReasoning {
    input_hash: hash(&task.data),
    output_hash: hash(&result),
    prover: node_address,
};

// 4. Submit & get validated
submit_proof(proof).await?;
// → Multi-validator verification
// → Consensus reached
// → Rewards distributed (70% prover, 30% validators)
```

## 🛠️ Tech Stack

**Backend**
- Rust 1.90+ (Cognitive Engine, Validator Plugin)
- Tokio (Async runtime)
- Ethers-rs (Blockchain interaction)

**Smart Contracts**
- Solidity 0.8.20
- OpenZeppelin
- Hardhat

**Blockchain**
- Polygon (Layer 2)
- Bor (Validator client)

## 📁 Project Structure

```
polyneurons/
├── cognitive-engine/     # 🦀 Rust - AI reasoning engine
├── validator-plugin/     # 🦀 Rust - Validator integration  
├── por-consensus/        # 🦀 Rust - Consensus mechanism
├── shared/              # 🦀 Rust - Shared types
├── contracts/           # 📜 Solidity - Smart contracts
├── examples/            # 💡 Usage examples
├── docs/               # 📚 Documentation
└── test/               # 🧪 Tests
```

## 💰 Economics

### Reward Split
- **Block Reward**: 80% standard validation, 20% cognitive tasks
- **Task Reward**: 70% prover, 30% validators

### Stake Requirement
- Minimum: 100 MATIC
- Reputation-based task assignment
- Slashing for invalid proofs

## 🧪 Testing

```bash
# All tests
make test

# Rust only
cargo test --workspace

# Contracts only
npm run test
```

## 📚 Documentation

- 📖 [Quick Start Guide](QUICKSTART.md) - Get started in 5 minutes
- 🏗️ [Architecture](docs/architecture.md) - Detailed design & flow
- 🚀 [Deployment Guide](docs/deployment.md) - Production deployment
- 🧪 [Testnet Guide](docs/testnet-guide.md) - Polygon Amoy testnet setup
- 📘 [API Reference](docs/api.md) - Complete API docs
- 🤝 [Contributing](CONTRIBUTING.md) - Contribution guidelines

## 🚀 Deployment

### Testnet (Amoy)
```bash
npm run deploy:amoy
cargo run --example register_node
make run-engine
```

### Mainnet (Polygon)
```bash
npm run deploy:polygon
# Setup systemd services (see docs/deployment.md)
```

## 🔐 Security

- ✅ Stake requirement (100 MATIC minimum)
- ✅ Multi-validator consensus (3+ confirmations)
- ✅ Reputation system
- ✅ Slashing mechanism
- ✅ OpenZeppelin security standards

## 🌟 Features

- ✨ High-performance Rust implementation
- ✨ Gas-optimized Solidity contracts
- ✨ Modular & extensible architecture
- ✨ Comprehensive test coverage
- ✨ Production-ready deployment scripts
- ✨ Complete documentation

## 🤝 Contributing

Contributions welcome! Steps:

1. Fork repository
2. Create feature branch (`git checkout -b feature/amazing`)
3. Commit changes (`git commit -m 'feat: add amazing'`)
4. Push to branch (`git push origin feature/amazing`)
5. Open Pull Request

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## 📄 License

MIT License - see [LICENSE](LICENSE) file

## 🙏 Acknowledgments

- Polygon team for amazing L2 infrastructure
- Rust community for excellent tooling
- OpenZeppelin for security standards

---

**Built with** ❤️ **using Rust & Solidity**

[Documentation](docs/) • [Examples](examples/) • [Issues](https://github.com/your-org/polyneurons/issues)
