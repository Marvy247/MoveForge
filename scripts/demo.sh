#!/bin/bash

# ParaForge Demo Script - Clean Output Version
# Follow along with NARRATOR_SCRIPT.md for what to say and demonstrate

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$( cd "$SCRIPT_DIR/.." && pwd )"
PARAFORGE="$PROJECT_ROOT/target/release/paraforge"

GREEN='\033[1;32m'
CYAN='\033[0;36m'
NC='\033[0m'

# Check if binary exists
if [ ! -f "$PARAFORGE" ]; then
    echo "Error: ParaForge binary not found. Run: cargo build --release"
    exit 1
fi

clear
echo ""
echo -e "${GREEN}⚡ ParaForge Demo - Movement M1 Hackathon${NC}"
echo ""
read -p "[STEP 1] Press Enter to show CLI commands..."
clear
echo ""

# Step 1: CLI Overview
"$PARAFORGE" --help
echo ""
read -p "[STEP 2] Press Enter for parallel simulation..."
clear
echo ""

# Step 2: Parallel Simulation (10 transactions)
"$PARAFORGE" sim --parallel --count 10 --output table
echo ""
read -p "[STEP 3] Press Enter to scale up (100 txs)..."
clear
echo ""

# Step 3: Large Scale Simulation (100 transactions)
"$PARAFORGE" sim --parallel --count 100 --output table
echo ""
read -p "[STEP 4] Press Enter for testing suite..."
clear
echo ""

# Step 4: Testing Suite
"$PARAFORGE" test --parallel
echo ""
"$PARAFORGE" test --fuzz --fuzz-runs 100
echo ""
read -p "[STEP 5] Press Enter for AI analysis..."
clear
echo ""

# Step 5: AI Analysis
"$PARAFORGE" analyze "$PROJECT_ROOT/examples/contracts/defi_swap.move" --vuln --optimize
echo ""
read -p "[STEP 6] Press Enter to deploy contract..."
clear
echo ""

# Step 6: Deployment
"$PARAFORGE" deploy "$PROJECT_ROOT/examples/contracts/defi_swap.move" --network testnet
echo ""
read -p "[STEP 7] Press Enter to show dashboard..."
echo ""
echo -e "${CYAN}→ Open browser to: http://localhost:3000${NC}"
echo -e "${CYAN}→ Dashboard should be running (if not, open new terminal: cd frontend && npm run dev)${NC}"
echo ""
read -p "[STEP 8] Press Enter when done with dashboard demo..."
clear
echo ""

# Step 7: Project Init (Optional)
cd /tmp
"$PARAFORGE" init movement-defi --template hybrid
cd - > /dev/null
echo ""
read -p "[FINAL] Press Enter for summary..."
clear
echo ""

# Summary
echo -e "${GREEN}✅ Demo Complete!${NC}"
echo ""
echo "ParaForge - First parallel execution testing toolkit for Movement"
echo "GitHub: github.com/paraforge/paraforge"
echo "Documentation: README.md"
echo ""
