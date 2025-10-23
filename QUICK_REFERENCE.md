# PolyNeurons Quick Reference

## 🎯 Your Deployed Contracts (Amoy Testnet)

- **CognitiveRegistry**: `0x455d57610da2d5dB9e2aAac8A3c0A67beE607a29`
- **ProofOfReasoning**: `0x12afD7ad2c7D7eA3D7064f601010D5e209db51ab`
- **Network**: Polygon Amoy (Chain ID: 80002)
- **Explorer**: https://amoy.polygonscan.com/

## 💰 Get Test MATIC

You need at least **100 MATIC** to register as a cognitive node.

### Faucets
1. **Polygon Official**: https://faucet.polygon.technology/
2. **Alchemy**: https://www.alchemy.com/faucets/polygon-amoy
3. **QuickNode**: https://faucet.quicknode.com/polygon/amoy

Your wallet address: `0x3e4d881819768fab30c5a79F3A9A7e69f0a935a4`

## 🚀 Quick Commands

### Build & Test
```bash
# Build all components
make build

# Run tests
make test

# Clean build artifacts
make clean
```

### Run Services
```bash
# Start cognitive engine
make run-engine

# Start validator plugin
make run-validator
```

### Examples
```bash
# Register as cognitive node (requires 100 MATIC)
make register-node
# Or: cargo run -p polyneurons-examples --example register_node

# Submit a reasoning task
make submit-task
# Or: cargo run -p polyneurons-examples --example submit_task
```

### Contract Interaction

#### Check Your Balance
```bash
cast balance 0x3e4d881819768fab30c5a79F3A9A7e69f0a935a4 \
  --rpc-url https://rpc-amoy.polygon.technology
```

#### View Contract on Explorer
- Registry: https://amoy.polygonscan.com/address/0x455d57610da2d5dB9e2aAac8A3c0A67beE607a29
- PoR: https://amoy.polygonscan.com/address/0x12afD7ad2c7D7eA3D7064f601010D5e209db51ab

#### Fund PoR Contract
```bash
cast send 0x12afD7ad2c7D7eA3D7064f601010D5e209db51ab \
  "fundContract()" \
  --value 1ether \
  --private-key $PRIVATE_KEY \
  --rpc-url $AMOY_RPC_URL
```

#### Check Node Registration
```bash
cast call 0x455d57610da2d5dB9e2aAac8A3c0A67beE607a29 \
  "cognitiveNodes(address)(address,string,uint256,uint256,uint256,bool,uint256)" \
  0x3e4d881819768fab30c5a79F3A9A7e69f0a935a4 \
  --rpc-url $AMOY_RPC_URL
```

## 📊 System Status

### Check if Cognitive Engine is Running
```bash
ps aux | grep cognitive-engine
```

### View Logs
```bash
# If running in terminal, logs appear directly
# If using systemd:
sudo journalctl -u polyneurons-engine -f
```

## 🔧 Troubleshooting

### "Insufficient funds"
- Get more MATIC from faucets
- Each faucet has cooldown (usually 24h)
- Try multiple faucets

### "Connection refused"
- Check RPC URL in `.env`
- Try alternative RPC: `https://polygon-amoy.g.alchemy.com/v2/demo`

### "Example not found"
- Use: `cargo run -p polyneurons-examples --example <name>`
- Or use make commands: `make register-node`

### "Contract not deployed"
- Verify addresses in `.env` match deployed contracts
- Check on explorer: https://amoy.polygonscan.com/

## 📝 Environment Variables

Current configuration in `.env`:
```bash
AMOY_RPC_URL=https://rpc-amoy.polygon.technology
REGISTRY_ADDRESS=0x455d57610da2d5dB9e2aAac8A3c0A67beE607a29
POR_CONTRACT_ADDRESS=0x12afD7ad2c7D7eA3D7064f601010D5e209db51ab
```

## 🎯 Next Steps

1. ✅ Contracts deployed
2. ⏳ Get 100 MATIC from faucets
3. ⏳ Register as cognitive node
4. ⏳ Start cognitive engine
5. ⏳ Submit test tasks

## 📚 Documentation

- [Quick Start](QUICKSTART.md)
- [Architecture](docs/architecture.md)
- [API Reference](docs/api.md)
- [Testnet Guide](docs/testnet-guide.md)
- [Deployment](docs/deployment.md)

## 🆘 Need Help?

- Open an issue: https://github.com/your-org/polyneurons/issues
- Check logs for errors
- Verify network connectivity
- Ensure sufficient MATIC balance
