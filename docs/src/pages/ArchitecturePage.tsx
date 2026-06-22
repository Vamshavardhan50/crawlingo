import { motion } from 'framer-motion';
import { Layers, Server, Network, Database, Cpu, HelpCircle } from 'lucide-react';
import CodeBlock from '@/components/ui/CodeBlock';
import Callout from '@/components/ui/Callout';
import HoverBorderCard from '@/components/ui/HoverBorderCard';

const components = [
  {
    name: 'Session',
    icon: <Server size={16} />,
    description: 'Central config holder holding user-agent headers, session cookies, rate limit, timeout, proxy strings, and browser profile settings.',
  },
  {
    name: 'Fetcher Engine (wreq)',
    icon: <Network size={16} />,
    description: 'Binds dynamic connection pools, handles custom DNS resolution via hickory-resolver, and rotating browser TLS/JA3 fingerprints.',
  },
  {
    name: 'Streaming Parser (lol_html)',
    icon: <Cpu size={16} />,
    description: 'Fast streaming HTML parser from Cloudflare that processes bytes on the fly and constructs vector-indexed element structures.',
  },
  {
    name: 'Fuzzy Selector Matcher',
    icon: <Database size={16} />,
    description: 'Computes character-level tag matching and attribute weights using Jaro-Winkler distances to solve selector drift issues.',
  },
];

export default function ArchitecturePage() {
  return (
    <div className="space-y-12">
      {/* Title */}
      <motion.div
        className="pb-6 border-b border-gray-200 dark:border-white/10"
        initial={{ opacity: 0, y: 15 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.5 }}
      >
        <h1 className="text-display-sm md:text-display-md font-bold text-black dark:text-white tracking-tight mb-3">
          Architecture & Internals
        </h1>
        <p className="text-base text-gray-600 dark:text-gray-400 max-w-2xl leading-relaxed">
          Crawlingo is split into a core Rust library and native bindings targeting Python, Node.js, and Go. All heavy-lifting calculations are done directly inside Rust memory space.
        </p>
      </motion.div>

      {/* Visual Sequence Chart */}
      <div className="scroll-mt-20" id="sequence-flow">
        <h2 className="text-lg font-semibold text-black dark:text-white mb-6">
          Lifecycle Sequence Pipeline
        </h2>

        <div className="grid grid-cols-1 md:grid-cols-4 gap-4 relative">
          <HoverBorderCard className="p-5 flex flex-col justify-between h-full min-h-[140px]">
            <div>
              <span className="text-[10px] font-mono text-gray-400 dark:text-gray-500 uppercase font-semibold">Step 1</span>
              <h4 className="text-sm font-semibold text-black dark:text-white mt-1 mb-2">SDK Client</h4>
              <p className="text-xs text-gray-500 dark:text-gray-400 leading-relaxed">User initiates a Session script in Python, JS, or Go.</p>
            </div>
          </HoverBorderCard>

          <HoverBorderCard className="p-5 flex flex-col justify-between h-full min-h-[140px]">
            <div>
              <span className="text-[10px] font-mono text-gray-400 dark:text-gray-500 uppercase font-semibold">Step 2</span>
              <h4 className="text-sm font-semibold text-black dark:text-white mt-1 mb-2">Rust FFI Layer</h4>
              <p className="text-xs text-gray-500 dark:text-gray-400 leading-relaxed">Memory pointers are shared directly across Maturing/NAPI-RS boundaries.</p>
            </div>
          </HoverBorderCard>

          <HoverBorderCard className="p-5 flex flex-col justify-between h-full min-h-[140px]">
            <div>
              <span className="text-[10px] font-mono text-gray-400 dark:text-gray-500 uppercase font-semibold">Step 3</span>
              <h4 className="text-sm font-semibold text-black dark:text-white mt-1 mb-2">Parsing Engine</h4>
              <p className="text-xs text-gray-500 dark:text-gray-400 leading-relaxed">Lol-HTML builds DOM maps; Jaro-Winkler scoring repairs drifted elements.</p>
            </div>
          </HoverBorderCard>

          <HoverBorderCard className="p-5 flex flex-col justify-between h-full min-h-[140px]">
            <div>
              <span className="text-[10px] font-mono text-gray-400 dark:text-gray-500 uppercase font-semibold">Step 4</span>
              <h4 className="text-sm font-semibold text-black dark:text-white mt-1 mb-2">Arrow Output</h4>
              <p className="text-xs text-gray-500 dark:text-gray-400 leading-relaxed">Streams memory blocks out to CSV, Parquet, or JSON formats.</p>
            </div>
          </HoverBorderCard>
        </div>
      </div>

      {/* Core components list */}
      <div className="scroll-mt-20" id="core-components">
        <h2 className="text-lg font-semibold text-black dark:text-white mb-6">
          Core Engine Modules
        </h2>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {components.map((item, idx) => (
            <HoverBorderCard key={idx} className="p-6">
              <div className="flex items-center gap-2 mb-3">
                <div className="w-8 h-8 rounded-lg bg-black dark:bg-white text-white dark:text-black flex items-center justify-center">
                  {item.icon}
                </div>
                <h4 className="text-sm font-semibold text-black dark:text-white">{item.name}</h4>
              </div>
              <p className="text-xs text-gray-500 dark:text-gray-400 leading-relaxed">{item.description}</p>
            </HoverBorderCard>
          ))}
        </div>
      </div>

      {/* Rust Bindings */}
      <div className="scroll-mt-20" id="bindings">
        <h2 className="text-lg font-semibold text-black dark:text-white mb-4">
          Cross-Language Bindings
        </h2>
        <p className="text-sm text-gray-600 dark:text-gray-400 leading-relaxed mb-6">
          Crawlingo leverages modern bindings to run Rust-level speed while exposing native SDK features:
        </p>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div className="p-6 border border-gray-200 dark:border-white/10 rounded-2xl bg-white dark:bg-zinc-950">
            <h3 className="text-sm font-semibold text-black dark:text-white mb-2">Python Bindings (PyO3)</h3>
            <p className="text-xs text-gray-500 dark:text-gray-400 leading-relaxed">
              Exposed natively via the PyO3 crate and managed via Maturin. Data conversion operates directly on memory buffers using Apache Arrow, mapping Python Lists to raw C structs without serialization penalties.
            </p>
          </div>
          <div className="p-6 border border-gray-200 dark:border-white/10 rounded-2xl bg-white dark:bg-zinc-950">
            <h3 className="text-sm font-semibold text-black dark:text-white mb-2">Node.js Bindings (NAPI-RS)</h3>
            <p className="text-xs text-gray-500 dark:text-gray-400 leading-relaxed">
              Exposed natively through Javascript promises mapping to internal Tokio threads inside the Rust core. Fully typed definitions allow typescript projects to check properties natively.
            </p>
          </div>
        </div>
      </div>

      {/* Export Arrow Details */}
      <div className="scroll-mt-20" id="mem-management">
        <h2 className="text-lg font-semibold text-black dark:text-white mb-3">
          Memory Optimization
        </h2>
        <p className="text-sm text-gray-600 dark:text-gray-400 mb-6">
          To maintain a minimal footprint, datasets stream results as Apache Arrow record batches, bypassing Javascript or Python object allocation until explicitly requested by the SDK client.
        </p>

        <CodeBlock
          language="python"
          filename="streaming.py"
          showLineNumbers
          code={`# Streams batches directly from Rust heap to local file systems
# without allocating a full Python array.
dataset.build().to_parquet("output_stream.parquet")`}
        />
      </div>

      <Callout type="info" title="Need help with the classes?">
        Review the complete method definitions in the{' '}
        <a href="/api-reference" className="text-black dark:text-white hover:underline">
          API Reference
        </a>.
      </Callout>
    </div>
  );
}
