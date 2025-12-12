'use client';

import { useState, useEffect } from 'react';
import { Button } from '@/components/ui/button';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui/card';
import { SimulationPanel } from '@/components/simulation-panel';
import { TransactionList } from '@/components/transaction-list';
import { MetricsDisplay } from '@/components/metrics-display';
import { Zap, Activity, TrendingUp, Fuel, ArrowLeft } from 'lucide-react';
import Link from 'next/link';

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
      } else {
        setIsConnected(false);
      }
    } catch (error) {
      console.log('MoveForge node not running - using demo mode');
      setIsConnected(false);
    }
  };

  useEffect(() => {
    connectToNode().catch(() => {});
  }, []);

  return (
    <div className="min-h-screen bg-black">
      <header className="border-b border-gray-800 bg-zinc-950 backdrop-blur-lg sticky top-0 z-50">
        <div className="container mx-auto px-4 py-4">
          <div className="flex justify-between items-center">
            <div className="flex items-center gap-4">
              <Link href="/" className="flex items-center gap-2 text-gray-400 hover:text-white transition-colors">
                <ArrowLeft className="w-4 h-4" />
                <span className="text-sm">Back</span>
              </Link>
              <div className="flex items-center gap-3">
                <Zap className="w-6 h-6 text-white" />
                <h1 className="text-2xl font-semibold text-white">MoveForge Dashboard</h1>
              </div>
              <div className="flex items-center gap-2 px-3 py-1.5 bg-zinc-900 rounded-lg border border-gray-800">
                <Activity className={`w-3.5 h-3.5 ${isConnected ? 'text-green-500' : 'text-yellow-500'}`} />
                <span className="text-xs text-gray-400 font-medium">
                  {isConnected ? 'Connected' : 'Demo Mode'}
                </span>
              </div>
            </div>
          </div>
        </div>
      </header>

      <div className="container mx-auto px-4 py-8">
        <div className="grid lg:grid-cols-3 gap-6 mb-8">
          <MetricCard
            icon={<Activity className="w-5 h-5" />}
            title="Total Transactions"
            value={simulations.length.toString()}
            change="+12%"
            positive
          />
          <MetricCard
            icon={<TrendingUp className="w-5 h-5" />}
            title="Success Rate"
            value="94.2%"
            change="+2.1%"
            positive
          />
          <MetricCard
            icon={<Fuel className="w-5 h-5" />}
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
          
          <Card className="bg-zinc-950 border-gray-800">
            <CardHeader className="border-b border-gray-800">
              <CardTitle className="text-white font-semibold">Recent Simulations</CardTitle>
              <CardDescription className="text-gray-400">View your latest simulation results</CardDescription>
            </CardHeader>
            <CardContent className="pt-6">
              <TransactionList transactions={simulations} />
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
}

function MetricCard({ 
  icon, 
  title, 
  value, 
  change, 
  positive 
}: { 
  icon: React.ReactNode;
  title: string; 
  value: string; 
  change: string; 
  positive: boolean;
}) {
  return (
    <Card className="bg-zinc-950 border-gray-800">
      <CardContent className="pt-6">
        <div className="flex items-start justify-between">
          <div>
            <p className="text-sm text-gray-400 mb-2">{title}</p>
            <h3 className="text-3xl font-bold text-white">{value}</h3>
            <p className={`text-sm mt-2 ${positive ? 'text-green-500' : 'text-red-500'}`}>
              {change}
            </p>
          </div>
          <div className="text-gray-400">
            {icon}
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
