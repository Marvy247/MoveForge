# @paraforge/sdk

TypeScript SDK for ParaForge - The Movement DevEx Toolkit

## Installation

```bash
npm install @paraforge/sdk
# or
yarn add @paraforge/sdk
```

## Quick Start

```typescript
import { ParaForgeClient, SimulationBuilder, TransactionBuilder } from '@paraforge/sdk';

// Initialize client
const client = new ParaForgeClient({
  nodeUrl: 'http://localhost:8545',
  network: 'testnet',
  parallel: true,
});

// Create and simulate transactions
const tx1 = TransactionBuilder.createTransfer(
  '0xalice',
  '0xbob',
  '1000000'
);

const tx2 = TransactionBuilder.createSwap(
  '0xalice',
  '0xpool',
  '500000',
  '0x...'
);

// Simulate in parallel
const results = await new SimulationBuilder(client)
  .addTransaction(tx1)
  .addTransaction(tx2)
  .setParallel(true)
  .run();

console.log('Results:', results);
console.log('Total Gas:', results.totalGasUsed);
console.log('Parallel Efficiency:', results.parallelEfficiency);
```

## Wallet Integration

```typescript
import { WalletConnector } from '@paraforge/sdk';

const wallet = new WalletConnector();

// Connect to Petra wallet
const address = await wallet.connect('petra');
console.log('Connected:', address);

// Sign transaction
const signedTx = await wallet.signTransaction(tx);
```

## Features

- 🚀 Parallel transaction simulation
- 💼 Multi-wallet support (Petra, Martian, MetaMask)
- 🔍 Gas estimation and optimization
- 🤖 AI-powered vulnerability detection
- 📊 Real-time analytics
- 🔗 Hybrid Move-EVM support

## API Reference

### ParaForgeClient

Main client for interacting with ParaForge node.

#### Methods

- `simulateTransaction(tx)` - Simulate a single transaction
- `simulateBatch(txs, parallel)` - Simulate multiple transactions
- `sendTransaction(tx)` - Send transaction to network
- `getBalance(address)` - Get account balance
- `deployContract(code, type)` - Deploy Move or Solidity contract
- `analyzeTransaction(hash)` - Get AI-powered analysis

### TransactionBuilder

Fluent builder for creating transactions.

#### Methods

- `from(address)` - Set sender address
- `to(address)` - Set recipient address
- `value(amount)` - Set transaction value
- `gasLimit(limit)` - Set gas limit
- `type(type)` - Set transaction type
- `build()` - Build the transaction

### SimulationBuilder

Builder for batch simulations.

#### Methods

- `addTransaction(tx)` - Add transaction to batch
- `addTransactions(txs)` - Add multiple transactions
- `setParallel(enabled)` - Enable/disable parallel execution
- `run()` - Execute simulation

## License

MIT
