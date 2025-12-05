import { Card, CardContent } from '@/components/ui/card';

interface MetricsDisplayProps {
  title: string;
  value: string | number;
  change?: string;
  positive?: boolean;
}

export function MetricsDisplay({ title, value, change, positive }: MetricsDisplayProps) {
  return (
    <Card className="bg-slate-900 border-slate-800 hover:border-purple-500/30 transition-all">
      <CardContent className="pt-6">
        <div className="text-sm text-gray-400 mb-2">{title}</div>
        <div className="text-3xl font-bold text-white mb-2">{value}</div>
        {change && (
          <div className={`text-sm flex items-center gap-1 ${positive ? 'text-green-400' : 'text-red-400'}`}>
            <span>{positive ? '↑' : '↓'}</span>
            <span>{change} from last period</span>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
