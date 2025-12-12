import Link from 'next/link';
import { Button } from '@/components/ui/button';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui/card';
import { Zap, ArrowRight, GitBranch, Search, Link2, Terminal, Rocket, Shield } from 'lucide-react';

export default function Home() {
  return (
    <main className="min-h-screen bg-black">
      {/* Hero Section */}
      <div className="container mx-auto px-4 py-20">
        <div className="text-center mb-20 space-y-6">
          <div className="inline-flex items-center gap-3 mb-6">
            <Zap className="w-12 h-12 text-white" strokeWidth={2} />
            <h1 className="text-7xl font-bold text-white tracking-tight">
              MoveForge
            </h1>
          </div>
          <p className="text-2xl text-gray-400 mb-6 font-light">
            Foundry-Grade Testing for Movement Blockchain
          </p>
          <p className="text-lg text-gray-500 max-w-2xl mx-auto leading-relaxed">
            Professional developer toolkit for parallel execution testing, local node forking, 
            and AI-powered contract analysis on Movement.
          </p>
        </div>
        {/* CTA Buttons */}
        <div className="flex justify-center gap-4 mb-20">
          <Link href="/dashboard">
            <Button 
              size="lg" 
              className="bg-white text-black hover:bg-gray-200 font-medium transition-all h-12 px-8"
            >
              Launch Dashboard
              <ArrowRight className="ml-2 w-4 h-4" />
            </Button>
          </Link>
          <Link href="/docs">
            <Button 
              size="lg" 
              variant="outline" 
              className="border-gray-700 text-white hover:bg-gray-900 h-12 px-8"
            >
              Documentation
            </Button>
          </Link>
        </div>

        {/* Feature Grid */}
        <div className="grid md:grid-cols-3 gap-8 mb-20 max-w-6xl mx-auto">
          <FeatureCard
            icon={<GitBranch className="w-10 h-10" />}
            title="Parallel Simulation"
            description="Test concurrent transactions at scale with real-time conflict detection and resource analysis"
          />
          <FeatureCard
            icon={<Search className="w-10 h-10" />}
            title="AI-Powered Analysis"
            description="Identify vulnerabilities and optimization opportunities automatically with integrated LLM support"
          />
          <FeatureCard
            icon={<Link2 className="w-10 h-10" />}
            title="Hybrid Move-EVM"
            description="Seamlessly test interactions between Move modules and EVM contracts in one environment"
          />
          <FeatureCard
            icon={<Terminal className="w-10 h-10" />}
            title="CLI-First Design"
            description="Powerful command-line interface inspired by Foundry for familiar developer experience"
          />
          <FeatureCard
            icon={<Rocket className="w-10 h-10" />}
            title="Network Integration"
            description="Direct connection to Movement testnet and mainnet with real-time RPC health monitoring"
          />
          <FeatureCard
            icon={<Shield className="w-10 h-10" />}
            title="Production Ready"
            description="Battle-tested with comprehensive examples, full documentation, and active development"
          />
        </div>


        {/* Quick Start Card */}
        <Card className="max-w-4xl mx-auto bg-zinc-950 border-gray-800">
          <CardHeader className="border-b border-gray-800">
            <div className="flex items-center gap-2">
              <Terminal className="w-5 h-5 text-gray-400" />
              <CardTitle className="text-2xl text-white font-semibold">Quick Start</CardTitle>
            </div>
            <CardDescription className="text-gray-400">
              Get started with MoveForge in minutes
            </CardDescription>
          </CardHeader>
          <CardContent className="pt-6">
            <pre className="bg-black p-6 rounded-lg text-gray-300 overflow-x-auto border border-gray-800 font-mono text-sm leading-relaxed">
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

        {/* Stats Section */}
        <div className="grid md:grid-cols-4 gap-8 mt-20 max-w-5xl mx-auto">
          <StatCard number="9,495" label="Transactions/sec" />
          <StatCard number="8.9x" label="Parallel Speedup" />
          <StatCard number="<500ms" label="RPC Response" />
          <StatCard number="100%" label="Open Source" />
        </div>
      </div>
    </main>
  );
}

function FeatureCard({ icon, title, description }: { 
  icon: React.ReactNode; 
  title: string; 
  description: string 
}) {
  return (
    <Card className="bg-zinc-950 border-gray-800 hover:border-gray-700 transition-all group">
      <CardContent className="pt-6">
        <div className="text-white mb-4 group-hover:scale-110 transition-transform">
          {icon}
        </div>
        <h3 className="text-xl font-semibold text-white mb-3">{title}</h3>
        <p className="text-gray-400 leading-relaxed">{description}</p>
      </CardContent>
    </Card>
  );
}

function StatCard({ number, label }: { number: string; label: string }) {
  return (
    <div className="text-center p-6 bg-zinc-950 border border-gray-800 rounded-lg">
      <div className="text-3xl font-bold text-white mb-2">{number}</div>
      <div className="text-sm text-gray-400 uppercase tracking-wide">{label}</div>
    </div>
  );
}
