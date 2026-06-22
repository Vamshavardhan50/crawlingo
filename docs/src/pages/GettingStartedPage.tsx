import { motion } from 'framer-motion';
import { CheckCircle, Info, ChevronRight, HelpCircle, Code } from 'lucide-react';
import CodeBlock from '@/components/ui/CodeBlock';
import Callout from '@/components/ui/Callout';
import Tabs from '@/components/ui/Tabs';
import HoverBorderCard from '@/components/ui/HoverBorderCard';

const prerequisites = [
  'Python 3.8+, Node.js 16+, Go 1.18+, or Rust 1.70+',
  'Standard package manager (pip, npm, go get, cargo)',
  'No native compilers needed — pre-compiled binaries are shipped',
];

export default function GettingStartedPage() {
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
          Quick Start Guide
        </h1>
        <p className="text-base text-gray-600 dark:text-gray-400 max-w-2xl leading-relaxed">
          Learn how to install Crawlingo, understand the document object model (DOM), configure session headers, and execute your first self-healing extract script.
        </p>
      </motion.div>

      {/* Concept: Visualizing the DOM */}
      <div className="scroll-mt-20" id="dom-concepts">
        <h2 className="text-lg font-semibold text-black dark:text-white mb-4">
          1. Understanding HTML DOM Structure
        </h2>
        <p className="text-sm text-gray-600 dark:text-gray-400 leading-relaxed mb-6">
          Websites are structured as a tree of nodes called the **Document Object Model (DOM)**. Traditional scrapers parse this tree using exact matching paths. Look at the visual hierarchy below to see how nodes are nested:
        </p>

        <div className="p-6 border border-gray-200 dark:border-white/10 rounded-2xl bg-gray-50/50 dark:bg-zinc-950 font-mono text-xs leading-relaxed text-gray-500 dark:text-gray-400 space-y-2">
          <div className="flex items-center gap-1.5 text-black dark:text-white">
            <span className="text-zinc-400">└─</span>
            <code>html</code>
          </div>
          <div className="pl-6 flex items-center gap-1.5">
            <span className="text-zinc-400">└─</span>
            <code>body</code>
          </div>
          <div className="pl-12 flex items-center gap-1.5 text-black dark:text-white font-semibold bg-black/5 dark:bg-white/5 py-1 px-2 rounded w-fit">
            <span className="text-zinc-400">└─</span>
            <code>div.content</code>
            <span className="text-[10px] text-zinc-500">(Parent Node)</span>
          </div>
          <div className="pl-20 flex items-center gap-1.5 text-green-600 dark:text-green-400 font-semibold bg-green-500/5 py-0.5 px-2 rounded w-fit">
            <span className="text-zinc-400">├─</span>
            <code>h1.product-title</code>
            <span className="text-[10px] text-zinc-500">(Child Target: "Smartphone Pro")</span>
          </div>
          <div className="pl-20 flex items-center gap-1.5 text-zinc-600 dark:text-zinc-400">
            <span className="text-zinc-400">└─</span>
            <code>span.price</code>
            <span className="text-[10px] text-zinc-500">(Sibling: "$999")</span>
          </div>
        </div>

        <p className="text-sm text-gray-600 dark:text-gray-400 leading-relaxed mt-4">
          When query selectors drift, Crawlingo uses the parent-child-sibling coordinate mapping (the DOM Fingerprint) to scan candidate siblings and tags, finding the target element even if its class name is changed.
        </p>
      </div>

      {/* Prerequisites */}
      <div className="scroll-mt-20" id="prerequisites">
        <h2 className="text-lg font-semibold text-black dark:text-white mb-4">
          2. Prerequisites
        </h2>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          {prerequisites.map((req, idx) => (
            <HoverBorderCard key={idx} className="p-4 flex flex-col justify-between h-full">
              <div className="flex items-start gap-3">
                <CheckCircle size={16} className="text-green-500 shrink-0 mt-0.5" />
                <span className="text-xs text-gray-600 dark:text-gray-400 leading-relaxed font-medium">{req}</span>
              </div>
            </HoverBorderCard>
          ))}
        </div>
      </div>

      {/* Installation */}
      <div className="scroll-mt-20" id="installation">
        <h2 className="text-lg font-semibold text-black dark:text-white mb-3">
          3. Installation
        </h2>
        <p className="text-sm text-gray-600 dark:text-gray-400 mb-6">
          Install the appropriate library package for your programming language:
        </p>

        <Tabs
          tabs={[
            {
              id: 'python',
              label: 'Python',
              content: (
                <div className="space-y-4">
                  <CodeBlock code="pip install crawlingo" filename="Terminal" />
                  <Callout type="info">
                    Downloads the wheel package matching your platform. Pre-compiled Rust binaries are embedded natively.
                  </Callout>
                </div>
              ),
            },
            {
              id: 'node',
              label: 'Node.js',
              content: (
                <div className="space-y-4">
                  <CodeBlock code="npm install @crawlingo/sdk" filename="Terminal" />
                  <Callout type="info">
                    Uses Node API (NAPI-RS) to bind Javascript promises straight to Rust async threads.
                  </Callout>
                </div>
              ),
            },
            {
              id: 'go',
              label: 'Go',
              content: (
                <div className="space-y-4">
                  <CodeBlock code="go get github.com/crawlingo/crawlingo-go" filename="Terminal" />
                  <Callout type="info">
                    Binds to the core C-shared dynamic library (`libcrawlingo.so` / `.dylib` / `.dll`) generated by Rust.
                  </Callout>
                </div>
              ),
            },
            {
              id: 'rust',
              label: 'Rust',
              content: (
                <div className="space-y-4">
                  <CodeBlock code="cargo add crawlingo" filename="Terminal" />
                  <Callout type="info">
                    Import direct Cargo crate to leverage optimal compile-time optimizations without any FFI layer overhead.
                  </Callout>
                </div>
              ),
            },
          ]}
        />
      </div>

      {/* Sessions & Configurations */}
      <div className="scroll-mt-20" id="sessions">
        <h2 className="text-lg font-semibold text-black dark:text-white mb-3">
          4. Creating Your First Session
        </h2>
        <p className="text-sm text-gray-600 dark:text-gray-400 mb-4">
          All HTTP calls and document extractions originate from a <code className="font-mono text-xs bg-gray-100 dark:bg-white/10 px-1 py-0.5 rounded text-black dark:text-white">Session</code>. The session maintains connections, proxy configuration, and stores the selector fingerprinter cache.
        </p>

        <Tabs
          tabs={[
            {
              id: 'python',
              label: 'Python',
              content: (
                <CodeBlock
                  language="python"
                  filename="main.py"
                  showLineNumbers
                  code={`from crawlingo import Session

# Initialize session
session = Session()

# Configure behaviors
session.timeout(15)              # Request timeout in seconds
session.auto_match(True)         # Enable self-healing
session.fetcher_tier("stealthy") # Activate browser stealth headers
session.rate_limit(3.0)          # Limit to 3 requests per second

print("Session initialized successfully.")`}
                />
              ),
            },
            {
              id: 'node',
              label: 'Node.js',
              content: (
                <CodeBlock
                  language="typescript"
                  filename="main.ts"
                  showLineNumbers
                  code={`import { Session } from '@crawlingo/sdk';

const session = new Session();
session.timeout(15);
session.autoMatch(true);
session.fetcherTier('stealthy');
session.rateLimit(3.0);

console.log('Session initialized.');`}
                />
              ),
            },
            {
              id: 'go',
              label: 'Go',
              content: (
                <CodeBlock
                  language="go"
                  filename="main.go"
                  showLineNumbers
                  code={`package main

import (
	"fmt"
	"github.com/crawlingo/crawlingo-go"
)

func main() {
	session := crawlingo.NewSession()
	session.SetTimeout(15)
	session.SetAutoMatch(true)
	session.SetFetcherTier("stealthy")
	session.SetRateLimit(3.0)

	fmt.Println("Session initialized.")
}`}
                />
              ),
            },
          ]}
        />
      </div>

      {/* Fetch & Elements */}
      <div className="scroll-mt-20" id="elements">
        <h2 className="text-lg font-semibold text-black dark:text-white mb-3">
          5. Fetching & Extracting Data
        </h2>
        <p className="text-sm text-gray-600 dark:text-gray-400 mb-4">
          Fetch pages to generate a <code className="font-mono text-xs bg-gray-100 dark:bg-white/10 px-1 py-0.5 rounded text-black dark:text-white">Page</code>, then use CSS, XPath, or anchors to select tags and inspect text attributes.
        </p>

        <Tabs
          tabs={[
            {
              id: 'python',
              label: 'Python',
              content: (
                <CodeBlock
                  language="python"
                  filename="scrape.py"
                  showLineNumbers
                  code={`from crawlingo import Session

session = Session()
page = session.page("https://example.com/products")

# Query title tag
title = page.title()
print(f"Page Title: {title}")

# Fetch element collection
products = page.css("div.product-card")
for item in products:
    name = item.css("h2.title").text()
    price = item.css("span.price-tag").text()
    link = item.css("a").attr("href")
    
    print(f"Product: {name} | Price: {price} | URL: {link}")`}
                />
              ),
            },
            {
              id: 'node',
              label: 'Node.js',
              content: (
                <CodeBlock
                  language="typescript"
                  filename="scrape.ts"
                  showLineNumbers
                  code={`import { Session } from '@crawlingo/sdk';

const session = new Session();
const page = await session.page("https://example.com/products");

console.log('Page Title:', await page.title());

const products = await page.css('div.product-card');
for (const item of products) {
  const name = await item.css('h2.title').text();
  const price = await item.css('span.price-tag').text();
  console.log('Product: ' + name + ' | Price: ' + price);
}`}
                />
              ),
            },
          ]}
        />
      </div>

      {/* Next actions */}
      <Callout type="success" title="Setup Complete!">
        You have successfully initialized a Session and executed a DOM fetch. Explore the{' '}
        <a href="/architecture" className="text-black dark:text-white font-medium hover:underline">
          Architecture
        </a>{' '}
        details to understand how the scheduler and self-healing algorithms operate under the hood.
      </Callout>
    </div>
  );
}
