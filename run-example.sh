#!/bin/bash

# Load environment variables from .env
if [ ! -f .env ]; then
    echo "❌ .env file not found"
    exit 1
fi

set -a
source .env
set +a

# Run the example
if [ -z "$1" ]; then
    echo "Usage: ./run-example.sh <example-name>"
    echo "Examples: register_node, submit_task"
    exit 1
fi

echo "🚀 Running example: $1"
echo "Network: Amoy (Chain ID: 80002)"
echo "RPC: $AMOY_RPC_URL"
echo ""

cargo run --release -p polyneurons-examples --example "$1"
