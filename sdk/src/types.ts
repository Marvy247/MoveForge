export interface ParaForgeConfig {
  nodeUrl: string;
  network: 'testnet' | 'mainnet' | 'local';
  parallel?: boolean;
}

export interface Transaction {
  id?: string;
  from: string;
  to: string;
  value: string;
  gasLimit?: number;
  data?: string;
  type?: 'transfer' | 'swap' | 'mint' | 'burn' | 'deploy' | 'call';
}

export interface SimulationResult {
  id: string;
  type: string;
  status: 'success' | 'failed' | 'reverted';
  gasUsed: number;
  executionTimeMs: number;
  resourceChanges: ResourceChange[];
  conflicts?: string[];
  logs?: Log[];
}

export interface ResourceChange {
  address: string;
  resource: string;
  change: 'create' | 'update' | 'delete' | 'increase' | 'decrease';
  data?: any;
}

export interface Log {
  address: string;
  topics: string[];
  data: string;
}

export interface BatchSimulationResult {
  results: SimulationResult[];
  totalGasUsed: number;
  parallelEfficiency: number;
  conflicts: ConflictAnalysis;
  duration: number;
}

export interface ConflictAnalysis {
  totalConflicts: number;
  conflicts: Array<{
    resource: string;
    conflictingTxs: number[];
    severity: 'low' | 'medium' | 'high';
  }>;
  recommendation: string;
}

export interface DeploymentResult {
  address: string;
  transactionHash: string;
  network: string;
  gasUsed: number;
}

export interface AnalysisResult {
  vulnerabilities: Vulnerability[];
  optimizations: Optimization[];
  gasAnalysis: GasAnalysis;
  aiInsights?: string[];
}

export interface Vulnerability {
  type: string;
  severity: 'low' | 'medium' | 'high' | 'critical';
  description: string;
  location?: string;
  recommendation: string;
}

export interface Optimization {
  type: string;
  description: string;
  potentialSavings: string;
  implementation: string;
}

export interface GasAnalysis {
  totalGasUsed: number;
  averageGasPerTx: number;
  breakdown: Array<{
    operation: string;
    gas: number;
    percentage: number;
  }>;
}
