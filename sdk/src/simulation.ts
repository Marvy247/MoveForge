import { ParaForgeClient } from './client';
import { Transaction, BatchSimulationResult } from './types';

export class SimulationBuilder {
  private client: ParaForgeClient;
  private transactions: Transaction[] = [];
  private parallel: boolean = true;

  constructor(client: ParaForgeClient) {
    this.client = client;
  }

  addTransaction(tx: Transaction): SimulationBuilder {
    this.transactions.push(tx);
    return this;
  }

  addTransactions(txs: Transaction[]): SimulationBuilder {
    this.transactions.push(...txs);
    return this;
  }

  setParallel(parallel: boolean): SimulationBuilder {
    this.parallel = parallel;
    return this;
  }

  async run(): Promise<BatchSimulationResult> {
    if (this.transactions.length === 0) {
      throw new Error('No transactions to simulate');
    }

    const result = await this.client.simulateBatch(
      this.transactions,
      this.parallel
    );

    return result;
  }

  reset(): SimulationBuilder {
    this.transactions = [];
    return this;
  }

  getTransactionCount(): number {
    return this.transactions.length;
  }
}
