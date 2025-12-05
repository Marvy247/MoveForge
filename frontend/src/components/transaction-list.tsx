import { Card, CardContent } from '@/components/ui/card';

interface Transaction {
  results: Array<{
    id: string;
    status: string;
    gas_used: number;
    execution_time_ms: number;
  }>;
  totalGasUsed: number;
  parallelEfficiency: number;
  duration: number;
}

interface TransactionListProps {
  transactions: Transaction[];
}

export function TransactionList({ transactions }: TransactionListProps) {
  if (transactions.length === 0) {
    return (
      <div className="text-center text-gray-500 py-12">
        <div className="text-4xl mb-4">🔍</div>
        <p className="text-lg">No simulations yet</p>
        <p className="text-sm text-gray-600 mt-2">Run your first simulation to see results here</p>
      </div>
    );
  }

  return (
    <div className="space-y-4">
      {transactions.slice(0, 5).map((sim, idx) => (
        <Card key={idx} className="bg-slate-950 border-slate-800 hover:border-purple-500/50 transition-all">
          <CardContent className="pt-4">
            <div className="flex justify-between items-start mb-3">
              <div>
                <div className="text-sm font-medium text-white">
                  Batch Simulation #{transactions.length - idx}
                </div>
                <div className="text-xs text-gray-500 mt-1">
                  {sim.results.length} transactions
                </div>
              </div>
              <div className="text-right">
                <div className="text-sm text-green-400 font-medium">
                  {sim.results.filter((r) => r.status === 'success').length} succeeded
                </div>
                <div className="text-xs text-gray-500 mt-1">
                  {sim.duration.toFixed(0)}ms
                </div>
              </div>
            </div>
            
            <div className="grid grid-cols-3 gap-3 pt-3 border-t border-slate-800">
              <div>
                <div className="text-xs text-gray-500 mb-1">Total Gas</div>
                <div className="font-mono text-sm text-white font-medium">
                  {sim.totalGasUsed.toLocaleString()}
                </div>
              </div>
              <div>
                <div className="text-xs text-gray-500 mb-1">Efficiency</div>
                <div className="font-mono text-sm text-purple-400 font-medium">
                  {sim.parallelEfficiency.toFixed(1)}%
                </div>
              </div>
              <div>
                <div className="text-xs text-gray-500 mb-1">TPS</div>
                <div className="font-mono text-sm text-blue-400 font-medium">
                  {(sim.results.length / (sim.duration / 1000)).toFixed(0)}
                </div>
              </div>
            </div>
          </CardContent>
        </Card>
      ))}
    </div>
  );
}
