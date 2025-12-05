import Link from 'next/link';
import { Button } from '@/components/ui/button';
import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui/card';

export default function Docs() {
  return (
    <div className="min-h-screen bg-slate-950">
      <header className="border-b border-slate-800 bg-slate-900/50 backdrop-blur-lg sticky top-0 z-50">
        <div className="container mx-auto px-4 py-4 flex justify-between items-center">
          <h1 className="text-2xl font-bold text-white">⚡ ParaForge Documentation</h1>
          <Link href="/">
            <Button variant="outline" size="sm">
              Back to Home
            </Button>
          </Link>
        </div>
      </header>

      <div className="container mx-auto px-4 py-16 max-w-4xl">
        <Card className="mb-8 bg-slate-900 border-slate-800">
          <CardHeader>
            <CardTitle className="text-white">Installation</CardTitle>
            <CardDescription>Get ParaForge installed and running</CardDescription>
          </CardHeader>
          <CardContent>
            <pre className="bg-slate-950 p-4 rounded-lg overflow-x-auto text-green-400 border border-slate-800">
{`# Install ParaForge CLI
cargo install paraforge

# Or build from source
git clone https://github.com/paraforge/paraforge
cd paraforge
cargo build --release`}
            </pre>
          </CardContent>
        </Card>

        <Card className="mb-8 bg-slate-900 border-slate-800">
          <CardHeader>
            <CardTitle className="text-white">Quick Start</CardTitle>
            <CardDescription>Start using ParaForge in minutes</CardDescription>
          </CardHeader>
          <CardContent className="space-y-6">
            <div>
              <h3 className="text-lg font-semibold mb-3 text-white">1. Start Local Node</h3>
              <pre className="bg-slate-950 p-4 rounded-lg text-green-400 border border-slate-800">
{`paraforge node --fork testnet --port 8545`}
              </pre>
            </div>

            <div>
              <h3 className="text-lg font-semibold mb-3 text-white">2. Run Simulations</h3>
              <pre className="bg-slate-950 p-4 rounded-lg text-green-400 border border-slate-800">
{`paraforge sim --parallel --count 100 --web`}
              </pre>
            </div>

            <div>
              <h3 className="text-lg font-semibold mb-3 text-white">3. Test Contracts</h3>
              <pre className="bg-slate-950 p-4 rounded-lg text-green-400 border border-slate-800">
{`paraforge test --fuzz --parallel`}
              </pre>
            </div>

            <div>
              <h3 className="text-lg font-semibold mb-3 text-white">4. Deploy</h3>
              <pre className="bg-slate-950 p-4 rounded-lg text-green-400 border border-slate-800">
{`paraforge deploy contracts/MyModule.move --network testnet`}
              </pre>
            </div>
          </CardContent>
        </Card>

        <Card className="mb-8 bg-slate-900 border-slate-800">
          <CardHeader>
            <CardTitle className="text-white">SDK Usage</CardTitle>
            <CardDescription>Integrate ParaForge into your application</CardDescription>
          </CardHeader>
          <CardContent>
            <pre className="bg-slate-950 p-4 rounded-lg overflow-x-auto text-green-400 border border-slate-800">
{`import { ParaForgeClient, TransactionBuilder, SimulationBuilder } from '@paraforge/sdk';

const client = new ParaForgeClient({
  nodeUrl: 'http://localhost:8545',
  network: 'testnet',
});

const tx = TransactionBuilder.createTransfer(
  '0xalice',
  '0xbob',
  '1000000'
);

const results = await new SimulationBuilder(client)
  .addTransaction(tx)
  .setParallel(true)
  .run();`}
            </pre>
          </CardContent>
        </Card>

        <Card className="mb-8 bg-slate-900 border-slate-800">
          <CardHeader>
            <CardTitle className="text-white">Features</CardTitle>
            <CardDescription>What makes ParaForge powerful</CardDescription>
          </CardHeader>
          <CardContent>
            <ul className="space-y-3 text-gray-300">
              <li className="flex items-start gap-2">
                <span className="text-green-400">✓</span>
                <span>Parallel transaction simulation with conflict detection</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-green-400">✓</span>
                <span>Local node forking from Movement testnet/mainnet</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-green-400">✓</span>
                <span>Fuzzing and invariant testing</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-green-400">✓</span>
                <span>AI-powered vulnerability analysis</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-green-400">✓</span>
                <span>Gas optimization suggestions</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-green-400">✓</span>
                <span>Hybrid Move-EVM support</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-green-400">✓</span>
                <span>Real-time dashboard with transaction visualization</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-green-400">✓</span>
                <span>Multi-wallet integration (Petra, Martian, MetaMask)</span>
              </li>
            </ul>
          </CardContent>
        </Card>

        <Card className="bg-slate-900 border-slate-800">
          <CardHeader>
            <CardTitle className="text-white">Resources</CardTitle>
            <CardDescription>Learn more about ParaForge</CardDescription>
          </CardHeader>
          <CardContent>
            <ul className="space-y-3">
              <li>
                <a 
                  href="https://github.com/paraforge/paraforge" 
                  className="text-purple-400 hover:text-purple-300 hover:underline flex items-center gap-2"
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  <span>→</span>
                  <span>GitHub Repository</span>
                </a>
              </li>
              <li>
                <a 
                  href="https://developer.movementnetwork.xyz/" 
                  className="text-purple-400 hover:text-purple-300 hover:underline flex items-center gap-2"
                  target="_blank"
                  rel="noopener noreferrer"
                >
                  <span>→</span>
                  <span>Movement Developer Portal</span>
                </a>
              </li>
            </ul>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
