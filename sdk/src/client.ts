import axios, { AxiosInstance } from 'axios';
import {
  MoveForgeConfig,
  Transaction,
  SimulationResult,
  BatchSimulationResult,
  DeploymentResult,
  AnalysisResult,
} from './types';

export class MoveForgeClient {
  private api: AxiosInstance;
  private config: MoveForgeConfig;

  constructor(config: MoveForgeConfig) {
    this.config = config;
    this.api = axios.create({
      baseURL: config.nodeUrl,
      timeout: 30000,
      headers: {
        'Content-Type': 'application/json',
      },
    });
  }

  async simulateTransaction(tx: Transaction): Promise<SimulationResult> {
    const response = await this.api.post('/simulate', tx);
    return response.data;
  }

  async simulateBatch(
    transactions: Transaction[],
    parallel: boolean = true
  ): Promise<BatchSimulationResult> {
    const response = await this.api.post('/simulate/batch', {
      transactions,
      parallel: parallel ?? this.config.parallel ?? true,
    });
    
    return response.data;
  }

  async sendTransaction(tx: Transaction): Promise<string> {
    const response = await this.api.post('/eth_sendTransaction', [tx]);
    return response.data.result;
  }

  async getBalance(address: string): Promise<string> {
    const response = await this.api.post('/', {
      jsonrpc: '2.0',
      id: 1,
      method: 'eth_getBalance',
      params: [address, 'latest'],
    });
    return response.data.result;
  }

  async getBlockNumber(): Promise<number> {
    const response = await this.api.post('/', {
      jsonrpc: '2.0',
      id: 1,
      method: 'eth_blockNumber',
      params: [],
    });
    return parseInt(response.data.result, 16);
  }

  async deployContract(
    contractCode: string,
    contractType: 'move' | 'solidity'
  ): Promise<DeploymentResult> {
    const response = await this.api.post('/deploy', {
      code: contractCode,
      type: contractType,
      network: this.config.network,
    });
    
    return response.data;
  }

  async analyzeTransaction(
    txHash: string,
    options?: {
      vulnerabilityCheck?: boolean;
      optimizationSuggestions?: boolean;
    }
  ): Promise<AnalysisResult> {
    const response = await this.api.post('/analyze', {
      transactionHash: txHash,
      ...options,
    });
    
    return response.data;
  }

  async estimateGas(tx: Transaction): Promise<number> {
    const response = await this.api.post('/', {
      jsonrpc: '2.0',
      id: 1,
      method: 'eth_estimateGas',
      params: [tx],
    });
    return parseInt(response.data.result, 16);
  }

  async call(tx: Transaction, block: string = 'latest'): Promise<string> {
    const response = await this.api.post('/', {
      jsonrpc: '2.0',
      id: 1,
      method: 'eth_call',
      params: [tx, block],
    });
    return response.data.result;
  }

  getNodeUrl(): string {
    return this.config.nodeUrl;
  }

  getNetwork(): string {
    return this.config.network;
  }
}
