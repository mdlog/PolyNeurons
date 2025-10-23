# Quick Start Guide

Get PolyNeurons running in 5 minutes! ⚡

## Prerequisites

```bash
# Check Rust
rustc --version  # Should be 1.70+

# Check Node.js
node --version   # Should be 18+

# Check Cargo
cargo --version
```

## Installation

```bash
# Clone repository
git clone https://github.com/your-org/polyneurons
cd polyneurons

# Install all dependencies
make install

# Build project
make build
```

## Local Development

### 1. Start Local Blockchain

```bash
# Terminal 1: Start Hardhat node
npx hardhat node
```

### 2. Deploy Contracts

```bash
# Terminal 2: Deploy contracts
npm run deploy

# Copy contract addresses to .env
cp .env.example .env
# Edit .env with addresses from deploy output
```

### 3. Run Cognitive Engine

```bash
# Terminal 3: Start cognitive engine
POLYGON_RPC_URL=http://localhost:8545 \
PRIVATE_KEY=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 \
REGISTRY_ADDRESS=0x... \
cargo run --bin cognitive-engine
```

### 4. Run Validator Plugin

```bash
# Terminal 4: Start validator plugin
POLYGON_RPC_URL=http://localhost:8545 \
PRIVATE_KEY=0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d \
REGISTRY_ADDRESS=0x... \
cargo run --bin validator-plugin
```

## Test the System

### Register Node

```bash
cargo run --example register_node
```

Output:
```
🔌 Registering Cognitive Node...
📝 Node ID: polyneuron-1234567890
💰 Stake: 100 MATIC
✅ Node registered!
   Tx Hash: 0x...
```

### Submit Task

```bash
cargo run --example submit_task
```

Output:
```
📋 Submitting Reasoning Task...
🎯 Task Type: market_prediction
📊 Data Hash: 0x...
✅ Task submitted!
   Tx Hash: 0x...
   💰 Reward: 1 MATIC
```

### Watch Logs

```bash
# Cognitive Engine will automatically:
# 1. Detect new tasks
# 2. Process reasoning
# 3. Submit results
# 4. Generate Proof of Reasoning

# Validator Plugin will:
# 1. Validate blocks
# 2. Execute cognitive tasks
# 3. Verify peer proofs
```

## Run Tests

```bash
# Test all components
make test

# Test Rust only
cargo test --workspace

# Test contracts only
npm run test
```

## Deploy to Testnet (Amoy)

### 1. Setup Wallet

```bash
# Get testnet MATIC from faucet
# https://faucet.polygon.technology/

# Update .env
AMOY_RPC_URL=https://rpc-amoy.polygon.technology
PRIVATE_KEY=your_private_key_here
POLYGONSCAN_API_KEY=your_api_key
```

### 2. Deploy Contracts

```bash
npm run deploy:amoy
```

### 3. Register Node

```bash
cargo run --example register_node
```

### 4. Run Services

```bash
# Cognitive Engine
cargo run --release --bin cognitive-engine

# Validator Plugin
cargo run --release --bin validator-plugin
```

## Common Issues

### "Insufficient stake"
- Ensure you stake at least 100 MATIC
- Check balance: `cast balance $YOUR_ADDRESS --rpc-url $POLYGON_RPC_URL`

### "Connection refused"
- Ensure RPC URL is correct
- Check network connectivity
- Verify Hardhat node is running (for local)

### "Task not found"
- Ensure contract address is correct in .env
- Verify task has been created
- Check task hasn't expired

## Next Steps

1. 📖 Read [Architecture Documentation](docs/architecture.md)
2. 🔧 Explore [API Reference](docs/api.md)
3. 🚀 Follow [Deployment Guide](docs/deployment.md)
4. 🤝 Read [Contributing Guide](CONTRIBUTING.md)

## Support

- 📝 [Open an Issue](https://github.com/your-org/polyneurons/issues)
- 💬 [Join Discord](https://discord.gg/...)
- 📧 Email: support@polyneurons.io

## What's Next?

After the system is running, you can:

1. **Customize Reasoning Tasks**: Add new algorithms in `cognitive-engine/src/reasoning/`
2. **Optimize Performance**: Tune parameters for optimal performance
3. **Monitor Metrics**: Setup monitoring to track performance
4. **Scale Up**: Deploy multiple nodes for increased capacity

Happy coding! 🚀
