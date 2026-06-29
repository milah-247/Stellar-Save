#!/usr/bin/env bash
set -euo pipefail

# Build the stellar-save ROSCA contract to WASM.

cargo build \
  --manifest-path contracts/stellar-save/Cargo.toml \
  --target wasm32-unknown-unknown \
  --release

WASM="target/wasm32-unknown-unknown/release/stellar_save.wasm"
echo "✅ Build complete: $WASM"
