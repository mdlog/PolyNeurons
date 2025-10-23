#!/bin/bash

# Load environment variables
set -a
source .env
set +a

echo "🧪 Testing PolyNeurons Examples"
echo "================================"
echo ""
echo "Network: Polygon Amoy"
echo "RPC: $AMOY_RPC_URL"
echo "Registry: $REGISTRY_ADDRESS"
echo ""

# Check balance
echo "💰 Checking wallet balance..."
BALANCE=$(cast balance $REGISTRY_ADDRESS --rpc-url $AMOY_RPC_URL 2>/dev/null || echo "0")
echo "Balance: $BALANCE wei"
echo ""

# Test submit_task (requires less MATIC)
echo "📋 Testing submit_task example..."
echo "Note: This requires ~0.01 MATIC for gas"
echo ""
cargo run --release -p polyneurons-examples --example submit_task

echo ""
echo "================================"
echo "✅ Test complete!"
