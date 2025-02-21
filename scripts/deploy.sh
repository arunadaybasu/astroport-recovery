#!/bin/bash
# Deploy Governance Admin Change Contract

echo "Compiling contract..."
cargo build --release

echo "Deploying governance contract..."
terrad tx wasm store contracts/governance/admin_change.wasm --from wallet --gas auto
