#!/usr/bin/env bash
set -euo pipefail

# Deploy stellar-save to Stellar testnet.
# Prerequisites:
#   stellar keys generate deployer --network testnet
#   stellar keys fund deployer --network testnet

NETWORK="testnet"
IDENTITY="${IDENTITY:-deployer}"
WASM="target/wasm32-unknown-unknown/release/stellar_save.wasm"

echo "==> Building contract..."
bash "$(dirname "$0")/build.sh"

echo "==> Uploading WASM to $NETWORK..."
WASM_HASH=$(stellar contract upload \
  --network "$NETWORK" \
  --source "$IDENTITY" \
  --wasm "$WASM")

echo "WASM hash: $WASM_HASH"

echo "==> Deploying contract..."
CONTRACT_ID=$(stellar contract deploy \
  --network "$NETWORK" \
  --source "$IDENTITY" \
  --wasm-hash "$WASM_HASH")

echo "✅ Contract deployed: $CONTRACT_ID"
echo "   Network: $NETWORK"
echo ""
echo "To create your first ROSCA group:"
echo "  stellar contract invoke \\"
echo "    --network $NETWORK --source $IDENTITY \\"
echo "    --id $CONTRACT_ID \\"
echo "    -- create_group \\"
echo "    --contribution_amount 100000000 \\"
echo "    --cycle_duration 100 \\"
echo "    --max_members 5"
