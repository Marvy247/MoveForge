'use client';

import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui/card';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';

interface SimulationPanelProps {
  nodeUrl: string;
  onSimulation: (result: any) => void;
}

export function SimulationPanel({ nodeUrl, onSimulation }: SimulationPanelProps) {
  const [txCount, setTxCount] = useState(10);
  const [parallel, setParallel] = useState(true);
  const [loading, setLoading] = useState(false);

  const runSimulation = async () => {
    setLoading(true);
    
    try {
      const transactions = Array.from({ length: txCount }, (_, i) => ({
        id: `tx_${i}`,
        from: `0x${(i * 1000).toString(16).padStart(40, '0')}`,
        to: `0x${(i * 1000 + 1).toString(16).padStart(40, '0')}`,
        value: `${(i + 1) * 1000000}`,
        type: ['transfer', 'swap', 'mint'][i % 3],
      }));

      const simulatedResults = transactions.map((tx, i) => ({
        ...tx,
        status: Math.random() > 0.1 ? 'success' : 'failed',
        gas_used: 21000 + i * 100,
        execution_time_ms: Math.random() * 50 + 10,
      }));

      const result = {
        results: simulatedResults,
        totalGasUsed: simulatedResults.reduce((sum, r) => sum + r.gas_used, 0),
        parallelEfficiency: parallel ? 85 + Math.random() * 10 : 50,
        duration: Math.random() * 500 + 200,
      };

      setTimeout(() => {
        onSimulation(result);
        setLoading(false);
      }, 1000);
    } catch (error) {
      console.error('Simulation failed:', error);
      setLoading(false);
    }
  };

  return (
    <Card className="bg-slate-900 border-slate-800">
      <CardHeader>
        <CardTitle className="text-white">Run Simulation</CardTitle>
        <CardDescription>Configure and execute parallel transaction simulation</CardDescription>
      </CardHeader>
      <CardContent className="space-y-4">
        <div className="space-y-2">
          <Label htmlFor="txCount" className="text-white">
            Number of Transactions
          </Label>
          <Input
            id="txCount"
            type="number"
            value={txCount}
            onChange={(e) => setTxCount(parseInt(e.target.value) || 1)}
            min="1"
            max="1000"
            className="bg-slate-950 border-slate-700 text-white"
          />
        </div>

        <div className="flex items-center gap-2">
          <input
            type="checkbox"
            id="parallel"
            checked={parallel}
            onChange={(e) => setParallel(e.target.checked)}
            className="w-4 h-4 rounded border-slate-700 bg-slate-950 text-purple-600 focus:ring-purple-500"
          />
          <Label htmlFor="parallel" className="text-white cursor-pointer">
            Enable Parallel Execution
          </Label>
        </div>

        <Button
          onClick={runSimulation}
          disabled={loading}
          className="w-full bg-purple-600 hover:bg-purple-700"
        >
          {loading ? (
            <span className="flex items-center gap-2">
              <svg className="animate-spin h-4 w-4" viewBox="0 0 24 24">
                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" fill="none" />
                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
              </svg>
              Running Simulation...
            </span>
          ) : (
            'Run Simulation'
          )}
        </Button>

        <Card className="bg-slate-950 border-slate-800">
          <CardContent className="pt-4">
            <div className="text-sm text-gray-400 space-y-1">
              <p>
                This will simulate {txCount} transactions{' '}
                {parallel ? 'in parallel' : 'sequentially'}
              </p>
              <p className="text-xs text-gray-500">
                Expected time: ~{parallel ? Math.ceil(txCount / 10) : txCount * 0.1}s
              </p>
            </div>
          </CardContent>
        </Card>
      </CardContent>
    </Card>
  );
}
