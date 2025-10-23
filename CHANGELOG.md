# Changelog

All notable changes to PolyNeurons will be documented in this file.

## [0.1.0] - 2024-10-24

### Added
- Initial release of PolyNeurons
- Rust-based cognitive engine for reasoning tasks
- Validator plugin for Polygon network integration
- Proof of Reasoning (PoR) consensus mechanism
- Smart contracts for node registration and task management
- Support for 3 cognitive task types:
  - Market Prediction
  - Anomaly Detection
  - Risk Scoring
- Complete documentation suite
- Example implementations
- Test suite for Rust and Solidity components

### Changed
- **BREAKING**: Migrated from Mumbai testnet to Amoy testnet
  - Updated RPC URL to `https://rpc-amoy.polygon.technology`
  - Changed Chain ID from 80001 to 80002
  - Updated all documentation and configuration files
  - Updated faucet links and network details

### Technical Details
- Rust 1.75.0+ required
- Solidity 0.8.20
- Polygon Amoy testnet (Chain ID: 80002)
- Node.js 18+ for smart contract development

### Documentation
- Quick Start Guide
- Architecture Documentation
- API Reference
- Deployment Guide
- Testnet Guide (Amoy)
- Contributing Guidelines

### Security
- Stake requirement: 100 MATIC minimum
- Multi-validator consensus (3+ confirmations)
- Reputation-based task assignment
- Slashing mechanism for invalid proofs

---

## Migration Guide: Mumbai → Amoy

If you were using Mumbai testnet, follow these steps:

### 1. Update Environment Variables

```bash
# Old
MUMBAI_RPC_URL=https://rpc-mumbai.maticvigil.com

# New
AMOY_RPC_URL=https://rpc-amoy.polygon.technology
```

### 2. Update Network Configuration

In `hardhat.config.js`:
```javascript
// Old
mumbai: {
  url: process.env.MUMBAI_RPC_URL,
  chainId: 80001
}

// New
amoy: {
  url: process.env.AMOY_RPC_URL,
  chainId: 80002
}
```

### 3. Get New Test Tokens

- Visit: https://faucet.polygon.technology/
- Select "Polygon Amoy"
- Request test MATIC

### 4. Redeploy Contracts

```bash
npm run deploy:amoy
```

### 5. Update Contract Addresses

Update `.env` with new contract addresses from deployment output.

---

## Future Roadmap

### v0.2.0 (Planned)
- Additional reasoning task types
- Enhanced reputation system
- Gas optimization improvements
- Advanced monitoring dashboard

### v0.3.0 (Planned)
- Cross-chain support
- Decentralized task marketplace
- Advanced ML model integration
- Performance optimizations

### v1.0.0 (Planned)
- Production-ready mainnet deployment
- Full security audit
- Comprehensive monitoring suite
- Enterprise features

---

For more information, see:
- [Documentation](docs/)
- [GitHub Issues](https://github.com/your-org/polyneurons/issues)
- [Contributing Guide](CONTRIBUTING.md)
