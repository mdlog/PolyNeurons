# Deployment Guide

## Prerequisites

### System Requirements
- Ubuntu 20.04+ atau macOS
- 8GB RAM minimum
- 100GB SSD storage
- Rust 1.70+
- Node.js 18+

### Install Dependencies

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Clone repository
git clone https://github.com/your-org/polyneurons
cd polyneurons
```

## Setup

### 1. Install Dependencies

```bash
make install
```

### 2. Configure Environment

```bash
cp .env.example .env
nano .env
```

Update with values:
```env
# For testnet
AMOY_RPC_URL=https://rpc-amoy.polygon.technology
# For mainnet
POLYGON_RPC_URL=https://polygon-rpc.com
PRIVATE_KEY=your_validator_private_key
POLYGONSCAN_API_KEY=your_api_key
```

### 3. Build Project

```bash
make build
```

## Deploy Smart Contracts

### Testnet (Amoy)

```bash
# Deploy to Amoy testnet
npx hardhat run scripts/deploy.js --network amoy
```

### Mainnet (Polygon)

```bash
# CAUTION: Real funds!
npx hardhat run scripts/deploy.js --network polygon
```

Update `.env` dengan contract addresses:
```env
REGISTRY_ADDRESS=0x...
POR_CONTRACT_ADDRESS=0x...
```

## Register Cognitive Node

```bash
# Stake 100 MATIC and register
cargo run --example register_node
```

## Run Services

### Option 1: Separate Terminals

```bash
# Terminal 1: Cognitive Engine
make run-engine

# Terminal 2: Validator Plugin
make run-validator
```

### Option 2: Systemd Services

Create `/etc/systemd/system/polyneurons-engine.service`:
```ini
[Unit]
Description=PolyNeurons Cognitive Engine
After=network.target

[Service]
Type=simple
User=validator
WorkingDirectory=/opt/polyneurons
Environment="POLYGON_RPC_URL=https://polygon-rpc.com"
Environment="PRIVATE_KEY=your_key"
ExecStart=/opt/polyneurons/target/release/cognitive-engine
Restart=always

[Install]
WantedBy=multi-user.target
```

Create `/etc/systemd/system/polyneurons-validator.service`:
```ini
[Unit]
Description=PolyNeurons Validator Plugin
After=network.target

[Service]
Type=simple
User=validator
WorkingDirectory=/opt/polyneurons
Environment="POLYGON_RPC_URL=https://polygon-rpc.com"
Environment="PRIVATE_KEY=your_key"
ExecStart=/opt/polyneurons/target/release/validator-plugin
Restart=always

[Install]
WantedBy=multi-user.target
```

Enable and start:
```bash
sudo systemctl enable polyneurons-engine
sudo systemctl enable polyneurons-validator
sudo systemctl start polyneurons-engine
sudo systemctl start polyneurons-validator
```

## Monitoring

### Check Logs

```bash
# Systemd logs
sudo journalctl -u polyneurons-engine -f
sudo journalctl -u polyneurons-validator -f
```

### Check Node Status

```bash
# Query smart contract
cast call $REGISTRY_ADDRESS \
  "cognitiveNodes(address)(address,string,uint256,uint256,uint256,bool,uint256)" \
  $YOUR_ADDRESS \
  --rpc-url $POLYGON_RPC_URL
```

### Metrics

Monitor:
- Tasks completed
- Reputation score
- Rewards earned
- Proof verification rate

## Troubleshooting

### Node Not Receiving Tasks

1. Check stake amount: `>= 100 MATIC`
2. Verify node is active
3. Check RPC connection
4. Review logs for errors

### Proof Validation Failing

1. Verify computation correctness
2. Check peer validator connectivity
3. Review consensus parameters
3. Ensure sufficient gas

### Low Rewards

1. Increase task completion rate
2. Improve reputation score
3. Optimize reasoning algorithms
4. Participate in more validations

## Security Best Practices

1. **Private Key**: Never commit to git
2. **Firewall**: Restrict RPC access
3. **Updates**: Keep dependencies updated
4. **Monitoring**: Set up alerts
5. **Backup**: Regular key backups

## Upgrade

```bash
git pull origin main
make build
sudo systemctl restart polyneurons-engine
sudo systemctl restart polyneurons-validator
```

## Uninstall

```bash
# Stop services
sudo systemctl stop polyneurons-*
sudo systemctl disable polyneurons-*

# Remove files
sudo rm /etc/systemd/system/polyneurons-*
sudo rm -rf /opt/polyneurons

# Withdraw stake from contract
cast send $REGISTRY_ADDRESS \
  "deactivateNode()" \
  --private-key $PRIVATE_KEY \
  --rpc-url $POLYGON_RPC_URL
```
