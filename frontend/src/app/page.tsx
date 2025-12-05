import Link from 'next/link';
import { Button } from '@/components/ui/button';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui/card';

export default function Home() {
  return (
    <main className="min-h-screen bg-gradient-to-br from-slate-950 via-purple-950 to-slate-950">
      <div className="container mx-auto px-4 py-16">
        <div className="text-center mb-16 space-y-4">
          <h1 className="text-6xl font-bold text-white mb-4 bg-clip-text text-transparent bg-gradient-to-r from-purple-400 to-pink-400">
            ⚡ MoveForge
          </h1>
          <p className="text-2xl text-purple-300 mb-4">
            Foundry-Grade Testing for Movement
          </p>
          <p className="text-lg text-gray-400 max-w-2xl mx-auto">
            The Foundry for Movement blockchain. Simulate, test, and deploy with confidence.
          </p>
        </div>

        <div className="grid md:grid-cols-3 gap-6 mb-16 max-w-6xl mx-auto">
          <FeatureCard
            icon="🚀"
            title="Parallel Simulation"
            description="Test concurrent transactions at scale with real-time conflict detection"
          />
          <FeatureCard
            icon="🔍"
            title="AI-Powered Analysis"
            description="Identify vulnerabilities and optimization opportunities automatically"
          />
          <FeatureCard
            icon="🔗"
            title="Hybrid Move-EVM"
            description="Seamlessly test interactions between Move modules and EVM contracts"
          />
        </div>

        <div className="flex justify-center gap-4 mb-16">
          <Link href="/dashboard">
            <Button size="lg" className="bg-purple-600 hover:bg-purple-700">
              Launch Dashboard
            </Button>
          </Link>
          <Link href="/docs">
            <Button size="lg" variant="outline">
              View Docs
            </Button>
          </Link>
        </div>

        <Card className="max-w-4xl mx-auto bg-slate-900/50 border-slate-800">
          <CardHeader>
            <CardTitle className="text-2xl text-white">Quick Start</CardTitle>
            <CardDescription>Get started with MoveForge in minutes</CardDescription>
          </CardHeader>
          <CardContent>
            <pre className="bg-slate-950 p-4 rounded-lg text-green-400 overflow-x-auto border border-slate-800">
{`# Install MoveForge CLI
cargo install moveforge

# Start local node with testnet fork
moveforge node --fork testnet

# Run parallel simulation
moveforge sim --parallel --count 100 --web

# Deploy contract
moveforge deploy contracts/MyModule.move --network testnet`}
            </pre>
          </CardContent>
        </Card>
      </div>
    </main>
  );
}

function FeatureCard({ icon, title, description }: { icon: string; title: string; description: string }) {
  return (
    <Card className="bg-slate-900/50 border-slate-800 hover:bg-slate-900/70 transition-all hover:border-purple-500/50">
      <CardContent className="pt-6">
        <div className="text-4xl mb-4">{icon}</div>
        <h3 className="text-xl font-bold text-white mb-2">{title}</h3>
        <p className="text-gray-400">{description}</p>
      </CardContent>
    </Card>
  );
}
