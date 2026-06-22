import { motion } from 'framer-motion';
import CodeBlock from '@/components/ui/CodeBlock';
import Callout from '@/components/ui/Callout';
import Tabs from '@/components/ui/Tabs';
import HoverBorderCard from '@/components/ui/HoverBorderCard';

const pythonFeatures = [
  'Session, Page, Element, ElementCollection mapping classes.',
  'Dataset, DatasetResult, Crawl, and Watch pipeline hooks.',
  'Direct Pandas DataFrame conversion via .df() interface.',
  'Syntax-supported context managers: with Session() as s.',
];

const nodeFeatures = [
  'Complete Typescript definitions compile-time validated.',
  'Tokio-backed asynchronous promise methods.',
  'Platform-specific native NAPI binary builds.',
  'Compatible with standard CommonJS and ESM imports.',
];

export default function SDKPage() {
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
          SDK Documentation
        </h1>
        <p className="text-base text-gray-600 dark:text-gray-400 max-w-2xl leading-relaxed">
          Detailed developer guides for our Python and Node.js SDK bindings, providing idiomatic class wraps around our shared Rust scraper core.
        </p>
      </motion.div>

      {/* Feature Details */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <HoverBorderCard className="p-6 h-full flex flex-col justify-between">
          <div>
            <div className="flex items-center gap-3 mb-4">
              <div className="w-10 h-10 rounded-lg bg-black text-white dark:bg-white dark:text-black flex items-center justify-center font-bold font-mono">
                Py
              </div>
              <div>
                <h3 className="text-sm font-semibold text-black dark:text-white">Python SDK</h3>
                <span className="text-[10px] font-mono text-gray-400">pip install crawlingo</span>
              </div>
            </div>
            <ul className="space-y-2">
              {pythonFeatures.map((f, i) => (
                <li key={i} className="text-xs text-gray-500 dark:text-gray-400 flex items-center gap-2">
                  <div className="w-1 h-1 rounded-full bg-black dark:bg-white shrink-0" />
                  {f}
                </li>
              ))}
            </ul>
          </div>
        </HoverBorderCard>

        <HoverBorderCard className="p-6 h-full flex flex-col justify-between">
          <div>
            <div className="flex items-center gap-3 mb-4">
              <div className="w-10 h-10 rounded-lg bg-black text-white dark:bg-white dark:text-black flex items-center justify-center font-bold font-mono">
                JS
              </div>
              <div>
                <h3 className="text-sm font-semibold text-black dark:text-white">Node.js SDK</h3>
                <span className="text-[10px] font-mono text-gray-400">npm install @crawlingo/sdk</span>
              </div>
            </div>
            <ul className="space-y-2">
              {nodeFeatures.map((f, i) => (
                <li key={i} className="text-xs text-gray-500 dark:text-gray-400 flex items-center gap-2">
                  <div className="w-1 h-1 rounded-full bg-black dark:bg-white shrink-0" />
                  {f}
                </li>
              ))}
            </ul>
          </div>
        </HoverBorderCard>
      </div>

      {/* Code Snip sections */}
      <section className="scroll-mt-20">
        <h2 className="text-lg font-bold text-black dark:text-white mb-4">Basic Scrapes</h2>
        <Tabs
          tabs={[
            {
              id: 'python',
              label: 'Python',
              content: (
                <CodeBlock
                  language="python"
                  filename="simple.py"
                  showLineNumbers
                  code={`from crawlingo import Session

session = Session()
session.auto_match(True)

# Query tags
page = session.page("https://example.com")
print(page.css("h1").first().text())`}
                />
              ),
            },
            {
              id: 'node',
              label: 'Node.js',
              content: (
                <CodeBlock
                  language="typescript"
                  filename="simple.ts"
                  showLineNumbers
                  code={`import { Session } from '@crawlingo/sdk';

const session = new Session();
session.autoMatch(true);

const page = await session.page("https://example.com");
console.log((await page.css("h1")).first()?.text);`}
                />
              ),
            },
          ]}
        />
      </section>

      <section className="scroll-mt-20">
        <h2 className="text-lg font-bold text-black dark:text-white mb-4">Multi-Page Crawling</h2>
        <Tabs
          tabs={[
            {
              id: 'python',
              label: 'Python',
              content: (
                <CodeBlock
                  language="python"
                  filename="crawl.py"
                  showLineNumbers
                  code={`from crawlingo import Session

session = Session()
crawl = session.crawl("https://example.com/products")
crawl.follow("a.next-page")
crawl.limit(30)
crawl.field("title", "h1")

results = crawl.build()
for res in results:
    print(res["title"])`}
                />
              ),
            },
            {
              id: 'node',
              label: 'Node.js',
              content: (
                <CodeBlock
                  language="typescript"
                  filename="crawl.ts"
                  showLineNumbers
                  code={`import { Session, Crawl } from '@crawlingo/sdk';

const session = new Session();
const crawl = new Crawl("https://example.com/products", session);
crawl.follow("a.next-page");
crawl.limit(30);
crawl.field("title", "h1");

const results = await crawl.run();
for (const res of results) {
  console.log(res["title"]);
}`}
                />
              ),
            },
          ]}
        />
      </section>

      <section className="scroll-mt-20">
        <h2 className="text-lg font-bold text-black dark:text-white mb-4">Advanced Configuration (Proxy Pools, Webhooks & Scheduling)</h2>
        <Tabs
          tabs={[
            {
              id: 'python-adv',
              label: 'Python',
              content: (
                <CodeBlock
                  language="python"
                  filename="advanced.py"
                  showLineNumbers
                  code={`from crawlingo import Session

# 1. Custom similarity weights & proxy rotation pool
session = Session()
session.auto_match_weights({
    "text": 3.0,
    "class": 1.0,
    "depth": 0.5
})
session.proxy_pool([
    "http://proxy1.example.com:8080",
    "http://proxy2.example.com:8080"
])

crawl = session.crawl("https://example.com/products")
crawl.follow("a.next-page")
crawl.field("title", "h1")

# 2. Configure real-time webhook endpoints
crawl.webhook("https://my-api.com/webhooks/crawl")

# 3. Schedule recurring crawl loops in background (interval in seconds)
crawl.schedule(3600)  # every 1 hour

# Or run immediate synchronous crawl
# results = crawl.build()`}
                />
              ),
            },
            {
              id: 'node-adv',
              label: 'Node.js',
              content: (
                <CodeBlock
                  language="typescript"
                  filename="advanced.ts"
                  showLineNumbers
                  code={`import { Session, Crawl } from '@crawlingo/sdk';

// 1. Custom similarity weights & proxy rotation pool
const session = new Session();
session.autoMatchWeights({
  text: 3.0,
  class: 1.0,
  depth: 0.5
});
session.proxyPool([
  "http://proxy1.example.com:8080",
  "http://proxy2.example.com:8080"
]);

const crawl = new Crawl("https://example.com/products", session);
crawl.follow("a.next-page");
crawl.field("title", "h1");

// 2. Configure real-time webhook endpoints
crawl.webhook("https://my-api.com/webhooks/crawl");

// 3. Schedule recurring crawl loops in background (interval in seconds)
crawl.schedule(3600); // every 1 hour

// Or run immediate synchronous crawl
// const results = await crawl.run();`}
                />
              ),
            },
          ]}
        />
      </section>

      <Callout type="info">
        Both libraries use dynamic runtime allocations to ensure minimal data duplication when mapping DOM lists.
      </Callout>
    </div>
  );
}
