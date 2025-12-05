# ParaForge Demo Video Script

**Duration**: 3-5 minutes  
**Target Audience**: Hackathon judges, Movement developers

---

## Scene 1: The Problem (30 seconds)

**[Screen: Code editor with Movement contract]**

**Voiceover**: 
> "Movement's hybrid MoveVM plus EVM architecture is incredibly powerful, but testing concurrent transactions is a nightmare. There's no local forking, no parallel execution simulator, and hybrid Move-to-EVM interactions are nearly impossible to debug. Developers waste hours on deploy-test-debug cycles."

**[Show clips of frustration: slow deployments, terminal errors]**

---

## Scene 2: Introducing ParaForge (20 seconds)

**[Screen: ParaForge logo animation]**

**Voiceover**:
> "Meet ParaForge - the Foundry-inspired DevEx toolkit built specifically for Movement. Test parallel transactions, fork testnet state locally, and catch bugs before they hit production."

**[Show: Clean terminal with ParaForge logo]**

---

## Scene 3: Live Demo - Part 1 (60 seconds)

### Start Local Node

**[Terminal]**

```bash
$ paraforge node --fork testnet --port 8545
```

**[Output shows]**:
```
⚡ ParaForge - Movement DevEx Toolkit

Starting ParaForge local node...
📡 Forking from: https://aptos.testnet.porto.movementlabs.xyz/v1
⚡ Parallel execution: ENABLED
🌐 RPC endpoint: http://localhost:8545

✅ Node is running!
```

**Voiceover**:
> "First, we spin up a local ParaForge node that forks Movement testnet. This gives us real blockchain state locally - just like Anvil for Ethereum."

---

### Parallel Simulation

**[Terminal]**

```bash
$ paraforge sim --parallel --count 100 --web
```

**[Screen splits: Terminal + Web Dashboard opening]**

**Terminal Output**:
```
Running transaction simulation...

📦 Loaded 100 transactions
⚡ Parallel mode: ENABLED

✅ Simulation complete!
⏱️  Total time: 1.42s
🚀 Transactions/sec: 70

🌐 Launching web dashboard...
📊 Dashboard available at: http://localhost:3001
```

**[Web Dashboard shows]**:
- Transaction timeline with parallel execution visualization
- Gas usage graphs
- Conflict detection highlighting
- Success/failure status

**Voiceover**:
> "Now we simulate 100 concurrent transactions in parallel. ParaForge automatically detects resource conflicts and shows us exactly which transactions would fail - before we deploy anything."

---

## Scene 4: Live Demo - Part 2 (60 seconds)

### Hybrid Contract Interaction

**[Screen: Code editor showing Move module + Solidity contract]**

```move
// Move module calling EVM contract
module 0x1::bridge {
    public fun call_evm_contract() {
        // Hybrid bridge logic
    }
}
```

```solidity
// Solidity contract called by Move
contract EVMBridge {
    function processFromMove(bytes data) external {
        // Handle Move call
    }
}
```

**Voiceover**:
> "Here's where ParaForge shines - testing hybrid interactions. We have a Move module calling a Solidity contract through Movement's bridge."

**[Terminal]**

```bash
$ paraforge test tests/hybrid_bridge.rs --parallel
```

**[Output]**:
```
Running ParaForge test suite...

Running test_move_to_evm... ✓ PASSED
Running test_evm_to_move... ✓ PASSED
Running test_bridge_reentrancy... ✓ PASSED
Running test_concurrent_bridges... ✓ PASSED

─────────────────────────────
Total: 4 | Passed: 4 | Failed: 0

🎉 All tests passed!
```

**Voiceover**:
> "All tests pass, including concurrent bridge calls that would be impossible to test without ParaForge."

---

## Scene 5: AI Analysis (45 seconds)

**[Terminal]**

```bash
$ paraforge analyze 0x1::defi_protocol --vuln --optimize
```

**[Output with animations]**:
```
🤖 Analyzing with AI insights...

✓ Connected to local Ollama instance

🔍 Vulnerability Analysis:
  • No reentrancy vulnerabilities detected
  • Warning: Unchecked external call in transfer()
  • Resource race condition in concurrent mint

⚡ Gas Optimization Suggestions:
  • Batch transfers to save ~15% gas
  • Cache storage reads in loop (saves ~2000 gas)
  • Use events instead of storage for history

💡 AI Recommendations:
  1. Use Move's resource model for atomic ops
  2. Implement retry logic for conflicts
  3. Add access control checks
```

**Voiceover**:
> "ParaForge integrates with local LLMs for AI-powered analysis. It catches a resource race condition and suggests gas optimizations - all running locally with no API costs."

---

## Scene 6: Real Deployment (30 seconds)

**[Terminal]**

```bash
$ paraforge deploy contracts/defi_protocol.move --network testnet
```

**[Output]**:
```
Deploying contract to Movement...

📄 Contract: defi_protocol.move
🔧 Type: Move Module
🌐 Network: testnet

Compiling contract...
✓ Compilation successful

Deploying...
✅ Deployment successful!

Contract address: 0x1234...5678
Transaction hash: 0xabcd...ef01

🔗 View on explorer: https://explorer.movementnetwork.xyz/txn/0xabcd...
```

**Voiceover**:
> "With confidence from our simulations, we deploy to Movement testnet. The transaction goes through successfully."

---

## Scene 7: Dashboard Overview (20 seconds)

**[Screen: Web dashboard tour]**

**Show**:
1. Metrics cards (Total TXs, Success Rate, Gas)
2. Simulation panel with controls
3. Transaction list with details
4. Visual charts and graphs

**Voiceover**:
> "The web dashboard provides real-time visualization of all simulations, with detailed metrics on gas usage, parallel efficiency, and conflict analysis."

---

## Scene 8: SDK Integration (25 seconds)

**[Screen: VSCode with TypeScript code]**

```typescript
import { ParaForgeClient, TransactionBuilder } from '@paraforge/sdk';

const client = new ParaForgeClient({
  nodeUrl: 'http://localhost:8545',
  network: 'testnet',
});

const tx = TransactionBuilder.createSwap(
  '0xalice',
  '0xpool',
  '1000000',
  swapData
);

const result = await client.simulateTransaction(tx);
console.log('Gas:', result.gasUsed);
```

**Voiceover**:
> "Integrate ParaForge into any TypeScript app with our SDK. Perfect for frontends, bots, or testing frameworks."

---

## Scene 9: Closing (20 seconds)

**[Screen: ParaForge logo with features list]**

**Features overlay**:
- ⚡ Parallel Execution Testing
- 🔀 Local Testnet Forking
- 🔗 Hybrid Move-EVM Support
- 🤖 AI-Powered Analysis
- 📊 Real-time Dashboard
- 🧪 Advanced Testing Suite

**Voiceover**:
> "ParaForge brings Foundry-grade developer experience to Movement. Open source, production-ready, and built for the future of hybrid blockchain development."

**[Text on screen]**:
```
⚡ ParaForge
github.com/paraforge/paraforge
docs.paraforge.dev

Built for Movement M1 Hackathon 2024
```

**Voiceover**:
> "Ship faster, test smarter, build better - with ParaForge."

---

## Production Notes

### Visual Style
- Dark terminal theme (Dracula or Nord)
- Smooth transitions between scenes
- Highlight key outputs with colors/animations
- Use picture-in-picture for dashboard + terminal
- Add subtle background music (no lyrics)

### B-Roll Suggestions
- Code typing animations
- Network graphs showing parallel execution
- Transaction flow diagrams
- Movement blockchain visuals

### Recording Tools
- Terminal: [Asciinema](https://asciinema.org/) or OBS
- Screen recording: OBS Studio or ScreenFlow
- Editing: DaVinci Resolve (free) or Final Cut Pro
- Voiceover: Clear, enthusiastic, technical but accessible

### Upload Checklist
- [ ] Video rendered in 1080p minimum
- [ ] Closed captions added
- [ ] Upload to YouTube
- [ ] Add to Devpost submission
- [ ] Create thumbnail with ParaForge logo
- [ ] Add chapter markers in description
- [ ] Link to GitHub repo and docs

---

**Total Runtime**: ~4-5 minutes  
**Key Message**: ParaForge solves real Movement DevEx problems with production-ready tools.
