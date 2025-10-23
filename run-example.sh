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
    echo "Usage: ./run-example.sh <example-name> [args...]"
    echo "Examples: register_node, submit_task, view_task 2"
    exit 1
fi

EXAMPLE_NAME="$1"
shift  # Remove first argument, keep the rest

echo "🚀 Running example: $EXAMPLE_NAME"
echo "Network: Amoy (Chain ID: 80002)"
echo "RPC: $AMOY_RPC_URL"
echo ""

# Pass remaining arguments to the example
if [ $# -gt 0 ]; then
    cargo run --release -p polyneurons-examples --example "$EXAMPLE_NAME" -- "$@"
else
    cargo run --release -p polyneurons-examples --example "$EXAMPLE_NAME"
fi
