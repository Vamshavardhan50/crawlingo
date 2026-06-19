import { motion } from 'framer-motion';
import { ArrowRight, Layers, Zap, Shield, Database, Eye, GitBranch, AlertTriangle, RefreshCw } from 'lucide-react';
import { Link } from 'react-router-dom';
import CodeBlock from '@/components/ui/CodeBlock';
import Callout from '@/components/ui/Callout';
import EagleLogo from '@/components/ui/EagleLogo';
import { BentoGrid, BentoGridItem } from '@/components/ui/BentoGrid';
import HoverBorderCard from '@/components/ui/HoverBorderCard';

export default function IntroductionPage() {
  return (
    <div className="space-y-16">
      {/* Hero */}
      <div className="pb-8 border-b border-gray-200 dark:border-white/10">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6, ease: [0.16, 1, 0.3, 1] }}
        >
          <div className="mb-6 flex items-center gap-3">
            <motion.div
              className="inline-block"
              animate={{ rotateY: [0, 8, -8, 0], rotateX: [0, 3, -3, 0] }}
              transition={{ duration: 6, repeat: Infinity, ease: 'easeInOut' }}
              style={{ transformStyle: 'preserve-3d', perspective: '1000px' }}
            >
              <EagleLogo size="lg" />
            </motion.div>
            <span className="text-xs uppercase tracking-widest font-mono font-semibold px-2.5 py-1 bg-black text-white dark:bg-white dark:text-black rounded">
              v1.2.0 Stable
            </span>
          </div>

          <h1 className="text-display-lg md:text-display-xl mb-4 tracking-tight font-black text-black dark:text-white">
            Build Scrapers That <span className="underline decoration-wavy decoration-1">Survive Change</span>.
          </h1>

          <p className="text-lg md:text-xl text-gray-600 dark:text-gray-400 max-w-3xl leading-relaxed mb-8">
            Crawlingo is a Rust-powered web extraction framework designed to solve the fragile selector problem. By using DOM fingerprinting and fuzzy-match heuristics, Crawlingo automatically repairs broken selectors when websites update.
          </p>

          <div className="flex flex-wrap gap-4">
            <Link to="/getting-started" className="btn-primary group py-3 px-6 text-sm font-semibold">
              Get Started
              <ArrowRight size={16} className="transition-transform group-hover:translate-x-1" />
            </Link>
            <a
              href="https://github.com/crawlingo/crawlingo"
              target="_blank"
              rel="noopener noreferrer"
              className="btn-secondary py-3 px-6 text-sm font-semibold"
            >
              View on GitHub
            </a>
          </div>
        </motion.div>
      </div>

      {/* The Core Problem section */}
      <div className="scroll-mt-20" id="the-problem">
        <h2 className="text-display-sm font-bold text-black dark:text-white mb-6">
          The Fragile Selector Problem
        </h2>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-8 text-sm leading-relaxed text-gray-600 dark:text-gray-400">
          <div className="space-y-4">
            <p>
              In traditional web scraping, scripts rely on static identifiers like <code className="font-mono bg-gray-100 dark:bg-white/10 px-1 py-0.5 rounded text-black dark:text-white">id</code> attributes, CSS classes, or absolute XPath routes. When site developers update their code, rename selectors for CSS modules, or restructure their layout, those selectors instantly break.
            </p>
            <p>
              This causes scrapers to fail silently or throw exceptions, leading to broken data pipelines and requiring human intervention to find the new selector, rewrite the code, and redeploy the script.
            </p>
          </div>
          <div className="space-y-4">
            <div className="p-5 border border-red-500/20 bg-red-500/5 rounded-2xl flex gap-3">
              <AlertTriangle className="text-red-500 shrink-0" size={20} />
              <div>
                <span className="font-semibold text-black dark:text-white block mb-1">Traditional Drifting</span>
                <p className="text-xs text-gray-500 dark:text-gray-400">
                  A class modification like <code className="font-mono">.btn-primary</code> to <code className="font-mono">.btn-primary-v2</code> breaks standard libraries immediately, resulting in empty outputs or crash loops.
                </p>
              </div>
            </div>
            <div className="p-5 border border-green-500/20 bg-green-500/5 rounded-2xl flex gap-3">
              <Zap className="text-green-500 shrink-0" size={20} />
              <div>
                <span className="font-semibold text-black dark:text-white block mb-1">Crawlingo Self-Healing</span>
                <p className="text-xs text-gray-500 dark:text-gray-400">
                  Crawlingo scans the updated DOM, compares structural features (tags, content length, child-sibling ratio, and text attributes) using the Jaro-Winkler algorithm, and selects the matching candidate automatically.
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Feature Bento Grid */}
      <div className="scroll-mt-20" id="key-pillars">
        <h2 className="text-display-sm font-bold text-black dark:text-white mb-6">
          Key Pillars of Crawlingo
        </h2>
        <BentoGrid>
          <BentoGridItem
            title="Session-Based Control"
            icon={<Layers size={20} />}
            description="Configure cookies, headers, proxy pools, and rate-limits once at the session level. The Rust engine handles connection reuse and keeps sessions stateless."
          />
          <BentoGridItem
            title="Auto-Match Self-Healing"
            icon={<Zap size={20} />}
            description="Uses Jaro-Winkler similarity matching on element attributes and tags to heal broken selectors. Keeps scrapers working across website updates."
          />
          <BentoGridItem
            title="Stealth Fetcher Engine"
            icon={<Shield size={20} />}
            description="Impersonates standard browser TLS fingerprints and HTTP/2 headers using wreq. Prevents bot protection triggers without heavy headless overhead."
          />
          <BentoGridItem
            title="Stream & Format Exports"
            icon={<Database size={20} />}
            description="Directly stream large data extracts into JSON, CSV, Parquet, or load them into Pandas and DuckDB queries natively with low memory footprint."
          />
          <BentoGridItem
            title="Dynamic Watch Mode"
            icon={<Eye size={20} />}
            description="Define periodic page monitors to trigger callback events when DOM sub-elements (like prices or stocks) modify, using light cache comparison."
          />
          <BentoGridItem
            title="AI Model Context (MCP)"
            icon={<GitBranch size={20} />}
            description="Built-in Model Context Protocol server lets LLM agents run structured crawls, query elements, and extract schema details automatically."
          />
        </BentoGrid>
      </div>

      {/* Comparison Table */}
      <div className="scroll-mt-20" id="comparison">
        <h2 className="text-display-sm font-bold text-black dark:text-white mb-6">
          How Crawlingo Compares
        </h2>
        <div className="overflow-x-auto border border-gray-200 dark:border-white/10 rounded-2xl bg-white dark:bg-zinc-950">
          <table className="w-full text-left border-collapse text-xs md:text-sm">
            <thead>
              <tr className="bg-gray-50 dark:bg-zinc-900 border-b border-gray-200 dark:border-white/10 font-mono text-gray-500 dark:text-gray-400">
                <th className="p-4 font-semibold">Feature</th>
                <th className="p-4 font-semibold text-black dark:text-white">Crawlingo</th>
                <th className="p-4 font-semibold">BeautifulSoup</th>
                <th className="p-4 font-semibold">Scrapy</th>
                <th className="p-4 font-semibold">Playwright</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-200 dark:divide-white/10 text-gray-600 dark:text-gray-400">
              <tr className="hover:bg-gray-50 dark:hover:bg-white/5 transition-colors">
                <td className="p-4 font-medium text-black dark:text-white">Self-Healing Selectors</td>
                <td className="p-4 text-green-500 font-semibold flex items-center gap-1"><Zap size={14} /> Yes (Automatic)</td>
                <td className="p-4">No</td>
                <td className="p-4">No</td>
                <td className="p-4">No</td>
              </tr>
              <tr className="hover:bg-gray-50 dark:hover:bg-white/5 transition-colors">
                <td className="p-4 font-medium text-black dark:text-white">Language / Engine</td>
                <td className="p-4 text-black dark:text-white font-medium">Rust Core (FFI)</td>
                <td className="p-4">Python (Pure)</td>
                <td className="p-4">Python (Twisted)</td>
                <td className="p-4">Node / Python (Browser)</td>
              </tr>
              <tr className="hover:bg-gray-50 dark:hover:bg-white/5 transition-colors">
                <td className="p-4 font-medium text-black dark:text-white">Memory Usage</td>
                <td className="p-4 text-green-500 font-semibold">Very Low (Streaming)</td>
                <td className="p-4">Medium</td>
                <td className="p-4">Medium</td>
                <td className="p-4">Extremely High</td>
              </tr>
              <tr className="hover:bg-gray-50 dark:hover:bg-white/5 transition-colors">
                <td className="p-4 font-medium text-black dark:text-white">Built-in Stealth TLS</td>
                <td className="p-4 text-green-500 font-semibold">Yes</td>
                <td className="p-4">No</td>
                <td className="p-4">No (requires plugins)</td>
                <td className="p-4">Yes (with browser headers)</td>
              </tr>
              <tr className="hover:bg-gray-50 dark:hover:bg-white/5 transition-colors">
                <td className="p-4 font-medium text-black dark:text-white">Built-in MCP Server</td>
                <td className="p-4 text-green-500 font-semibold">Yes</td>
                <td className="p-4">No</td>
                <td className="p-4">No</td>
                <td className="p-4">No</td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      {/* Quick Setup */}
      <div className="scroll-mt-20" id="installation">
        <h2 className="text-display-sm font-bold text-black dark:text-white mb-4">
          Quick Installation
        </h2>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <HoverBorderCard className="p-6">
            <span className="text-xs font-semibold text-gray-400 uppercase tracking-widest font-mono">Python SDK</span>
            <p className="text-xs text-gray-500 dark:text-gray-400 mt-2 mb-4">Integrate high-speed self-healing scraping into Python applications.</p>
            <CodeBlock code="pip install crawlingo" filename="Terminal" />
          </HoverBorderCard>
          <HoverBorderCard className="p-6">
            <span className="text-xs font-semibold text-gray-400 uppercase tracking-widest font-mono">Node.js SDK</span>
            <p className="text-xs text-gray-500 dark:text-gray-400 mt-2 mb-4">Native bindings for fast parsing and dataset builds in JS/TS.</p>
            <CodeBlock code="npm install @crawlingo/sdk" filename="Terminal" />
          </HoverBorderCard>
        </div>
        <Callout type="info" className="mt-4">
          Crawlingo packages ship with pre-compiled Rust binaries. No local Rust compilation toolchain is required.
        </Callout>
      </div>

      {/* Quick Example */}
      <div className="scroll-mt-20" id="quick-example">
        <h2 className="text-display-sm font-bold text-black dark:text-white mb-4">
          Quick Example
        </h2>
        <p className="text-sm text-gray-600 dark:text-gray-400 mb-6">
          Set up a session, query element attributes, and export structured data in a few lines of code:
        </p>
        <CodeBlock
          language="python"
          filename="quick_scrape.py"
          showLineNumbers
          code={`from crawlingo import Session

# Create a session with default stealth settings and self-healing active
session = Session()
session.auto_match(True)
session.timeout(30)

# Build an extraction schema dataset
dataset = session.dataset("https://news.ycombinator.com")
dataset.field("title", ".titleline > a", selector_type="css")
dataset.field("points", ".score", selector_type="css")

result = dataset.build()
print(f"Extracted {len(result)} items.")

# Export options
result.to_json("hn_posts.json")
result.to_csv("hn_posts.csv")`}
        />
      </div>

      {/* What's Next Navigation */}
      <div className="pt-8 border-t border-gray-200 dark:border-white/10 flex flex-col sm:flex-row justify-between items-center gap-4">
        <div className="text-center sm:text-left">
          <h4 className="text-sm font-semibold text-black dark:text-white">Ready to proceed?</h4>
          <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">Jump right into the Quick Start guide to build your first crawler.</p>
        </div>
        <Link to="/getting-started" className="btn-primary py-2 px-5 text-xs font-semibold flex items-center gap-1">
          Quick Start Guide
          <ArrowRight size={14} />
        </Link>
      </div>
    </div>
  );
}
