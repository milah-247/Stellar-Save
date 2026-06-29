#!/usr/bin/env bash
set -euo pipefail

# Run the stellar-save test suite.

cargo test \
  --manifest-path contracts/stellar-save/Cargo.toml \
  -- --nocapture "$@"
