import { motion } from 'framer-motion';
import CodeBlock from '@/components/ui/CodeBlock';
import Callout from '@/components/ui/Callout';
import HoverBorderCard from '@/components/ui/HoverBorderCard';

const integrations = [
  {
    name: 'Pandas DataFrames',
    id: 'pandas',
    description: 'Export scraped datasets directly into Pandas DataFrames for machine learning or analytical pipelines.',
    code: `from crawlingo import Session

session = Session()
dataset = session.dataset("https://example.com/products")
dataset.field("name", ".product-name")

result = dataset.build()
df = result.df() # Returns standard pandas.DataFrame
print(df.head())`,
  },
  {
    name: 'Apache Parquet',
    id: 'parquet',
    description: 'Export to column-oriented Parquet files natively using Rust Arrow record streams.',
    code: `# Stream raw memory blocks to Parquet files
result.to_parquet("output_data.parquet")`,
  },
  {
    name: 'Model Context Protocol (MCP)',
    id: 'mcp-server',
    description: 'Built-in Model Context Protocol server lets LLM agents crawl URLs, inspect headers, and build dynamic schemas.',
    code: `# Launch the MCP server from command line
# crawlingo mcp --host 127.0.0.1 --port 8000

# Available tools:
# - fetch_page: Returns URL body text & headers.
# - extract_data: Executes one-shot structural extraction.`,
  },
  {
    name: 'IP Proxy Rotation & Pools',
    id: 'proxy-rotation',
    description: 'Set up static lists of rotating proxy servers or integrate directly with proxy API providers.',
    code: `session = Session()

# Option A: Set a static list of proxies to rotate round-robin
session.proxy_pool([
    "http://proxy1.example.com:8080",
    "http://proxy2.example.com:8080"
])

# Option B: Fetch active proxies dynamically from a provider URL
session.proxy_provider("https://api.proxydomain.com/v1/list?key=auth")
session.rate_limit(2.0)`,
  },
  {
    name: 'Real-time Webhook Delivery',
    id: 'webhooks',
    description: 'Stream extracted page records to external webhooks instantly, avoiding huge local memory overhead.',
    code: `session = Session()
crawl = session.crawl("https://example.com/products")
crawl.follow("a.next-page")
crawl.field("title", "h1")

# Stream each page result to your ingest service
crawl.webhook("https://api.yourservice.com/webhooks/crawl")
crawl.build()`,
  },
];

export default function IntegrationsPage() {
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
          Third-Party Integrations
        </h1>
        <p className="text-base text-gray-600 dark:text-gray-400 max-w-2xl leading-relaxed">
          Connect your scraping sessions directly to Python analysis pipelines, big data storage files, or AI Agent protocols.
        </p>
      </motion.div>

      {/* Integrations Grid */}
      <div className="space-y-8">
        {integrations.map((item, idx) => (
          <HoverBorderCard key={idx} className="p-6">
            <h2 className="text-base font-bold text-black dark:text-white mb-2">{item.name}</h2>
            <p className="text-xs text-gray-500 dark:text-gray-400 mb-4">{item.description}</p>
            <CodeBlock
              language="python"
              filename={`${item.id}_integration.py`}
              code={item.code}
            />
          </HoverBorderCard>
        ))}
      </div>

      <Callout type="info">
        Need help setting up custom integration scripts? Read the Matplotlib or DuckDB tutorials in the advanced section.
      </Callout>
    </div>
  );
}
