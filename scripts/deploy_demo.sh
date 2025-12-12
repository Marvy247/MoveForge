#!/bin/bash

# MoveForge Deployment Demo Script
# This script demonstrates the deployment workflow

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}⚡ MoveForge - Movement M1 Hackathon Deployment Demo${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

echo -e "${YELLOW}📦 Building MoveForge...${NC}"
cargo build --release
echo -e "${GREEN}✓ Build complete${NC}"
echo ""

MOVEFORGE="$PROJECT_ROOT/target/release/moveforge"

if [ ! -f "$MOVEFORGE" ]; then
    echo "Error: moveforge binary not found"
    exit 1
fi

echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}Demo 1: Deploy DeFi Swap Module${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

"$MOVEFORGE" deploy examples/contracts/defi_swap.move --network testnet

echo ""
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}Demo 2: Deploy NFT Minting Module${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

"$MOVEFORGE" deploy examples/contracts/nft_mint.move --network testnet

echo ""
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}Demo 3: Parallel Execution Simulation${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""

echo "Creating sample transactions for concurrent NFT minting..."
cat > /tmp/moveforge_demo_txs.json << 'EOF'
[
  {
    "id": "mint_user_1",
    "type": "mint",
    "sender": "0x1",
    "function": "moveforge_nft::concurrent_mint::mint_nft"
  },
  {
    "id": "mint_user_2",
    "type": "mint",
    "sender": "0x2",
    "function": "moveforge_nft::concurrent_mint::mint_nft"
  },
  {
    "id": "mint_user_3",
    "type": "mint",
    "sender": "0x3",
    "function": "moveforge_nft::concurrent_mint::mint_nft"
  },
  {
    "id": "mint_user_4",
    "type": "mint",
    "sender": "0x4",
    "function": "moveforge_nft::concurrent_mint::mint_nft"
  },
  {
    "id": "mint_user_5",
    "type": "mint",
    "sender": "0x5",
    "function": "moveforge_nft::concurrent_mint::mint_nft"
  }
]
EOF

echo ""
echo -e "${YELLOW}Running parallel simulation with 5 concurrent NFT mints...${NC}"
"$MOVEFORGE" sim --file /tmp/moveforge_demo_txs.json --parallel --output table

echo ""
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✅ Demo Complete!${NC}"
echo -e "${CYAN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo -e "${GREEN}Key Features Demonstrated:${NC}"
echo "  ✓ Move module deployment workflow"
echo "  ✓ Movement testnet integration"
echo "  ✓ Parallel execution simulation"
echo "  ✓ Conflict detection in concurrent transactions"
echo "  ✓ Real-world use case: NFT minting"
echo ""
echo -e "${YELLOW}Next Steps:${NC}"
echo "  1. Install Movement/Aptos CLI to deploy to actual testnet"
echo "  2. Fund account from faucet"
echo "  3. Run: movement move publish --package-dir examples --named-addresses moveforge_nft=default"
echo "  4. Monitor transactions at: https://explorer.movementnetwork.xyz"
echo ""
echo -e "${CYAN}For more info: ${NC}https://docs.movementlabs.xyz"
echo ""
