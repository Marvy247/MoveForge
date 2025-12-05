'use client';

import { useState, useEffect } from 'react';
import { Button } from '@/components/ui/button';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui/card';
import { SimulationPanel } from '@/components/simulation-panel';
import { TransactionList } from '@/components/transaction-list';
import { MetricsDisplay } from '@/components/metrics-display';

export default function Dashboard() {
  const [isConnected, setIsConnected] = useState(false);
  const [nodeUrl, setNodeUrl] = useState('http://localhost:8545');
  const [simulations, setSimulations] = useState<any[]>([]);

  const connectToNode = async () => {
    try {
      const response = await fetch(nodeUrl, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          jsonrpc: '2.0',
          id: 1,
          method: 'eth_blockNumber',
          params: [],
        }),
      });
      
      if (response.ok) {
        setIsConnected(true);
      }
    } catch (error) {
      console.error('Connection failed:', error);
      setIsConnected(false);
    }
  };

  useEffect(() => {
    connectToNode();
  }, []);

  return (
    <div className="min-h-screen bg-slate-950">
      <header className="border-b border-slate-800 bg-slate-900/50 backdrop-blur-lg sticky top-0 z-50">
        <div className="container mx-auto px-4 py-4 flex justify-between items-center">
          <div className="flex items-center gap-4">
            <h1 className="text-2xl font-bold text-white">⚡ ParaForge Dashboard</h1>
            <div className="flex items-center gap-2 px-3 py-1.5 bg-slate-800 rounded-full">
              <div className={`w-2 h-2 rounded-full ${isConnected ? 'bg-green-500 animate-pulse' : 'bg-red-500'}`} />
              <span className="text-sm text-gray-300">
                {isConnected ? 'Connected' : 'Disconnected'}
              </span>
            </div>
          </div>
          <div className="flex gap-2">
            <Button variant="outline" size="sm" onClick={connectToNode}>
              {isConnected ? 'Reconnect' : 'Connect'}
            </Button>
          </div>
        </div>
      </header>

      <div className="container mx-auto px-4 py-8">
        <div className="grid lg:grid-cols-3 gap-6 mb-8">
          <MetricsDisplay
            title="Total Transactions"
            value={simulations.length}
            change="+12%"
            positive
          />
          <MetricsDisplay
            title="Success Rate"
            value="94.2%"
            change="+2.1%"
            positive
          />
          <MetricsDisplay
            title="Avg Gas Used"
            value="45,231"
            change="-8.5%"
            positive
          />
        </div>

        <div className="grid lg:grid-cols-2 gap-6">
          <SimulationPanel
            nodeUrl={nodeUrl}
            onSimulation={(result) => {
              setSimulations((prev) => [result, ...prev]);
            }}
          />
          
          <Card className="bg-slate-900 border-slate-800">
            <CardHeader>
              <CardTitle className="text-white">Recent Simulations</CardTitle>
              <CardDescription>View your latest simulation results</CardDescription>
            </CardHeader>
            <CardContent>
              <TransactionList transactions={simulations} />
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}
