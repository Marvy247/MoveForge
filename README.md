# ⚡ ParaForge

**The Ultimate Parallel Execution Testing Toolkit for Movement Blockchain**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![TypeScript](https://img.shields.io/badge/typescript-%23007ACC.svg?style=flat&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)

ParaForge is a Foundry/Anvil-inspired DevEx toolkit specifically built for Movement blockchain, enabling developers to simulate, test, and deploy with confidence through parallel execution testing, local node forking, and AI-powered analysis.

## 🎯 Problem Solved

Movement's hybrid MoveVM + EVM architecture is powerful but lacks developer tooling compared to mature EVM ecosystems. Developers face:

- **No local parallel execution testing** - Can't test concurrent transactions that exploit Movement's parallelism
- **Missing testnet forking** - No way to fork mainnet/testnet state for local testing like Anvil
- **Limited hybrid debugging** - Hard to test Move ↔ EVM contract interactions
- **Slow feedback loops** - Deploy-test-debug cycles are time-consuming

## ✨ Solution: ParaForge

ParaForge bridges the gap with:

🚀 **Parallel Execution Simulation** - Test hundreds of concurrent transactions with conflict detection  
🔀 **Local Node Forking** - Fork Movement testnet/mainnet for instant local testing  
🔗 **Hybrid Move-EVM Testing** - Seamlessly test interactions between Move modules and Solidity contracts  
🤖 **AI-Powered Analysis** - Vulnerability detection and gas optimization via local LLM integration  
⚡ **Real-time Dashboard** - Visual transaction timeline with resource diff tracking  
🧪 **Advanced Testing Suite** - Fuzzing, invariants, and cheatcodes for comprehensive testing

## 🏗️ Architecture

```
ParaForge/
├── cli/              # Rust CLI tool (like Foundry/Forge)
├── core/             # Core blockchain simulation engine
├── simulator/        # Parallel transaction simulator with rayon
├── sdk/              # TypeScript SDK for integration
├── frontend/         # Next.js dashboard for visualization
└── examples/         # Sample contracts and scripts
```

## 📦 Installation

### CLI Installation

```bash
# Install from crates.io (when published)
cargo install paraforge

# Or build from source
git clone https://github.com/paraforge/paraforge
cd paraforge
cargo build --release
```

### SDK Installation

```bash
npm install @paraforge/sdk
# or
yarn add @paraforge/sdk
```

## 🚀 Quick Start

### 1. Start Local Node

```bash
# Fork from Movement testnet
paraforge node --fork testnet --port 8545

# Or start fresh local node
paraforge node --port 8545
```

### 2. Run Parallel Simulations

```bash
# Simulate 100 transactions in parallel
paraforge sim --parallel --count 100 --web

# With custom transaction file
paraforge sim --file transactions.json --parallel --output json
```

### 3. Test Your Contracts

```bash
# Run tests with parallel execution
paraforge test --parallel

# With fuzzing
paraforge test --fuzz --fuzz-runs 10000
```

### 4. Deploy to Movement

```bash
# Deploy Move module
paraforge deploy contracts/MyModule.move --network testnet

# Deploy Solidity contract
paraforge deploy contracts/MyContract.sol --network testnet
```

### 5. AI-Powered Analysis

```bash
# Analyze transaction for vulnerabilities
paraforge analyze 0xabc123... --vuln --optimize
```

## 💻 SDK Usage

### Basic Setup

```typescript
import { ParaForgeClient, TransactionBuilder, SimulationBuilder } from '@paraforge/sdk';

const client = new ParaForgeClient({
  nodeUrl: 'http://localhost:8545',
  network: 'testnet',
  parallel: true,
});
```

### Create and Simulate Transactions

```typescript
// Create transactions
const tx1 = TransactionBuilder.createTransfer(
  '0xalice',
  '0xbob',
  '1000000'
);

const tx2 = TransactionBuilder.createSwap(
  '0xalice',
  '0xpool',
  '500000',
  '0xswapData'
);

// Simulate in parallel
const results = await new SimulationBuilder(client)
  .addTransaction(tx1)
  .addTransaction(tx2)
  .setParallel(true)
  .run();

console.log('Total Gas:', results.totalGasUsed);
console.log('Efficiency:', results.parallelEfficiency);
console.log('Conflicts:', results.conflicts);
```

### Wallet Integration

```typescript
import { WalletConnector } from '@paraforge/sdk';

const wallet = new WalletConnector();

// Connect Petra wallet
const address = await wallet.connect('petra');

// Sign and send transaction
const signedTx = await wallet.signTransaction(tx);
await client.sendTransaction(signedTx);
```

## 🎨 Dashboard

Launch the interactive dashboard:

```bash
cd frontend
npm install
npm run dev
```

Visit `http://localhost:3000` to access:
- Real-time simulation controls
- Transaction visualization timeline
- Gas usage analytics
- Conflict detection reports
- Performance metrics

## 🔧 CLI Commands

### Node Management

```bash
paraforge node [OPTIONS]
  --fork <NETWORK>    Fork from testnet/mainnet
  --port <PORT>       Port to run on (default: 8545)
  --parallel          Enable parallel execution
```

### Simulation

```bash
paraforge sim [OPTIONS]
  --parallel          Run in parallel mode
  --file <FILE>       Transaction file (JSON)
  --count <N>         Number of transactions
  --output <FORMAT>   Output format (json/table/web)
  --web              Launch web dashboard
```

### Testing

```bash
paraforge test [OPTIONS] [PATH]
  --fuzz             Enable fuzzing
  --fuzz-runs <N>    Number of fuzz iterations
  --parallel         Parallel test execution
```

### Deployment

```bash
paraforge deploy <CONTRACT> [OPTIONS]
  --network <NET>    Network (testnet/mainnet/local)
  --key <KEY>        Private key or wallet path
```

### Analysis

```bash
paraforge analyze <INPUT> [OPTIONS]
  --vuln             Vulnerability detection
  --optimize         Gas optimization suggestions
```

### Project Initialization

```bash
paraforge init <NAME> [OPTIONS]
  --template <TYPE>  Template (move/solidity/hybrid)
```

## 🎯 Use Cases

### 1. DeFi Protocol Testing
```bash
# Test flash loan exploit protection
paraforge sim --file defi-attacks.json --parallel
```

### 2. NFT Concurrent Minting
```bash
# Simulate 1000 users minting simultaneously
paraforge test tests/concurrent-mint.rs --parallel
```

### 3. Hybrid Contract Debugging
```bash
# Test Move module calling Solidity contract
paraforge sim --file hybrid-bridge.json --web
```

### 4. Security Auditing
```bash
# AI-powered vulnerability scan
paraforge analyze 0xcontract --vuln
```

## 🤖 AI Integration

ParaForge integrates with [Ollama](https://ollama.ai/) for local AI-powered analysis:

```bash
# Install Ollama
curl -fsSL https://ollama.ai/install.sh | sh

# Pull a model
ollama pull codellama

# ParaForge will automatically use it for analysis
paraforge analyze 0xabc... --vuln --optimize
```

## 🔗 Movement Integration

ParaForge connects to Movement networks:

- **Testnet**: `https://aptos.testnet.porto.movementlabs.xyz/v1`
- **Mainnet**: `https://mainnet.movementnetwork.xyz/v1`
- **Local**: `http://localhost:8545`

Compatible with:
- Movement CLI
- Aptos SDK
- Movement Explorer
- Petra & Martian wallets

## 🏆 Hackathon Submission

**Category**: Best New Devex Tool on Movement  
**Prize**: $5,000

### Innovation Highlights

1. **First parallel execution simulator** for Movement ecosystem
2. **Hybrid Move-EVM testing** in a single toolkit
3. **AI-powered insights** for security and optimization
4. **Foundry-like UX** familiar to EVM developers
5. **Production-ready MVP** with full documentation

### Demo Video

[Watch Demo on YouTube](https://youtube.com/placeholder)

### Live Demo

- **Dashboard**: https://paraforge-demo.vercel.app
- **Documentation**: https://docs.paraforge.dev

## 📊 Benchmarks

Performance on M1 Mac:

| Transactions | Sequential | Parallel | Speedup |
|-------------|-----------|----------|---------|
| 10          | 150ms     | 45ms     | 3.3x    |
| 100         | 1.2s      | 180ms    | 6.7x    |
| 1000        | 12.5s     | 1.4s     | 8.9x    |

## 💰 Monetization Strategy

### Open-Core Model
- **CLI**: Free and open-source (MIT license)
- **Cloud Premium**: Hosted simulations, team collaboration ($49/mo)
- **Enterprise**: Custom integrations, SLA support ($999/mo)

### Additional Revenue
- Movement ecosystem grants
- Integration partnerships
- Professional services (auditing)

## 🛣️ Roadmap

### Phase 1 (Hackathon MVP) ✅
- [x] Parallel simulation engine
- [x] Local node forking
- [x] CLI with all core commands
- [x] TypeScript SDK
- [x] Web dashboard
- [x] AI integration

### Phase 2 (Post-Hackathon)
- [ ] VSCode extension
- [ ] Gas profiling & optimization
- [ ] Advanced fuzzing strategies
- [ ] Multi-chain simulation
- [ ] Cloud deployment

### Phase 3 (Future)
- [ ] Formal verification integration
- [ ] MEV simulation tools
- [ ] Team collaboration features
- [ ] CI/CD integrations

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

```bash
git clone https://github.com/paraforge/paraforge
cd paraforge
cargo test
```

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

- Movement Labs for the amazing hybrid architecture
- Encode Club for organizing the hackathon
- Foundry team for UX inspiration
- Aptos for Move language tooling

## 📞 Contact

- **GitHub**: [@paraforge](https://github.com/paraforge)
- **Twitter**: [@paraforge_dev](https://twitter.com/paraforge_dev)
- **Discord**: [Join Community](https://discord.gg/paraforge)
- **Email**: hello@paraforge.dev

---

Built with ⚡ for the Movement M1 Hackathon 2024
