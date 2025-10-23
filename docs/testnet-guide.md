# Polygon Amoy Testnet Guide

## Overview

Polygon Amoy is the new testnet that replaced Mumbai. It provides a testing environment for Polygon applications before deploying to mainnet.

## Network Details

- **Network Name**: Polygon Amoy Testnet
- **RPC URL**: `https://rpc-amoy.polygon.technology`
- **Chain ID**: 80002
- **Currency Symbol**: MATIC
- **Block Explorer**: https://amoy.polygonscan.com/

## Getting Started

### 1. Add Amoy to MetaMask

**Manual Configuration:**
- Network Name: Polygon Amoy
- RPC URL: https://rpc-amoy.polygon.technology
- Chain ID: 80002
- Currency Symbol: MATIC
- Block Explorer: https://amoy.polygonscan.com/

**Or visit:** https://chainlist.org/?search=amoy

### 2. Get Test MATIC

**Faucet Options:**

1. **Official Polygon Faucet**
   - URL: https://faucet.polygon.technology/
   - Select "Polygon Amoy"
   - Enter your wallet address
   - Complete CAPTCHA
   - Receive 0.5 MATIC

2. **Alchemy Faucet**
   - URL: https://www.alchemy.com/faucets/polygon-amoy
   - Connect wallet or enter address
   - Receive test MATIC

3. **QuickNode Faucet**
   - URL: https://faucet.quicknode.com/polygon/amoy
   - Enter wallet address
   - Receive test tokens

### 3. Configure PolyNeurons for Amoy

Update your `.env` file:

```env
# Amoy Testnet Configuration
AMOY_RPC_URL=https://rpc-amoy.polygon.technology
PRIVATE_KEY=your_private_key_here
POLYGONSCAN_API_KEY=your_api_key_here

# Contract Addresses (update after deployment)
REGISTRY_ADDRESS=0x...
POR_CONTRACT_ADDRESS=0x...
```

### 4. Deploy to Amoy

```bash
# Compile contracts
npm run compile

# Deploy to Amoy
npm run deploy:amoy

# Verify contracts (optional)
npx hardhat verify --network amoy <CONTRACT_ADDRESS>
```

### 5. Register Your Node

```bash
# Make sure you have at least 100 test MATIC
cargo run --example register_node
```

### 6. Submit Test Tasks

```bash
# Submit a reasoning task
cargo run --example submit_task
```

## Differences from Mumbai

| Feature | Mumbai (Deprecated) | Amoy (Current) |
|---------|-------------------|----------------|
| Chain ID | 80001 | 80002 |
| RPC URL | rpc-mumbai.maticvigil.com | rpc-amoy.polygon.technology |
| Status | Deprecated (2024) | Active |
| Faucet | Discontinued | Available |
| Block Explorer | mumbai.polygonscan.com | amoy.polygonscan.com |

## Common Issues

### "Insufficient funds"
- Get more test MATIC from faucets
- Each faucet has cooldown periods (usually 24h)
- Try multiple faucets if needed

### "Network not responding"
- Check RPC URL is correct
- Try alternative RPC: `https://polygon-amoy.g.alchemy.com/v2/demo`
- Verify internet connection

### "Transaction failed"
- Ensure you have enough MATIC for gas
- Check contract addresses are correct
- Verify network is Amoy (Chain ID: 80002)

## Useful Resources

- **Amoy Faucet**: https://faucet.polygon.technology/
- **Block Explorer**: https://amoy.polygonscan.com/
- **Network Status**: https://status.polygon.technology/
- **Documentation**: https://docs.polygon.technology/
- **Discord Support**: https://discord.gg/polygon

## Testing Checklist

Before deploying to mainnet, test on Amoy:

- [ ] Deploy smart contracts
- [ ] Verify contracts on PolygonScan
- [ ] Register cognitive node
- [ ] Submit test reasoning tasks
- [ ] Verify task processing
- [ ] Check proof generation
- [ ] Validate reward distribution
- [ ] Test edge cases
- [ ] Monitor gas costs
- [ ] Review security

## Migration from Mumbai

If you were using Mumbai:

1. Update RPC URL in `.env`:
   ```env
   # Old
   MUMBAI_RPC_URL=https://rpc-mumbai.maticvigil.com
   
   # New
   AMOY_RPC_URL=https://rpc-amoy.polygon.technology
   ```

2. Update hardhat.config.js:
   ```javascript
   amoy: {
     url: process.env.AMOY_RPC_URL,
     accounts: process.env.PRIVATE_KEY ? [process.env.PRIVATE_KEY] : [],
     chainId: 80002
   }
   ```

3. Get new test MATIC from Amoy faucets

4. Redeploy contracts to Amoy

5. Update contract addresses in `.env`

## Gas Optimization Tips

Amoy uses the same gas model as Polygon mainnet:

- **Average Gas Price**: ~30 Gwei
- **Block Time**: ~2 seconds
- **Block Gas Limit**: 30M gas

Optimize your contracts:
- Use `calldata` instead of `memory` for external functions
- Pack storage variables efficiently
- Use events instead of storage when possible
- Batch operations when feasible

## Next Steps

After successful Amoy testing:

1. Review all test results
2. Optimize gas usage
3. Conduct security audit
4. Prepare mainnet deployment
5. Setup monitoring and alerts
6. Deploy to Polygon mainnet

---

**Need Help?**
- Open an issue: https://github.com/your-org/polyneurons/issues
- Join Discord: [link]
- Email: support@polyneurons.io
