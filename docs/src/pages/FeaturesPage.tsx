import { motion } from 'framer-motion';
import CodeBlock from '@/components/ui/CodeBlock';
import Callout from '@/components/ui/Callout';
import Tabs from '@/components/ui/Tabs';
import HoverBorderCard from '@/components/ui/HoverBorderCard';
import SelectorSimulator from '@/components/ui/SelectorSimulator';

export default function FeaturesPage() {
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
          Feature Catalog
        </h1>
        <p className="text-base text-gray-600 dark:text-gray-400 max-w-2xl leading-relaxed">
          Explore the engine's core capabilities, including fast selector caching, native regex matches, change watchers, and the auto-matching self-healing algorithm.
        </p>
      </motion.div>

      {/* Selectors Grid */}
      <div className="space-y-16">
        {/* CSS Selectors */}
        <section id="css-selectors" className="scroll-mt-20">
          <h2 className="text-lg font-bold text-black dark:text-white mb-2">CSS Selectors</h2>
          <p className="text-xs text-gray-500 dark:text-gray-400 mb-4 leading-relaxed">
            Query element lists using standard tag names, IDs, class paths, and hierarchy filters. Built-in caching avoids redundant page parsing.
          </p>
          <Tabs
            tabs={[
              {
                id: 'python',
                label: 'Python',
                content: (
                  <CodeBlock
                    language="python"
                    filename="css_selectors.py"
                    showLineNumbers
                    code={`page = session.page("https://example.com")

# Query tags
headings = page.css("h1")

# Class match
cards = page.css(".product-card")

# ID match
header = page.css("#main-header")`}
                  />
                ),
              },
              {
                id: 'node',
                label: 'Node.js',
                content: (
                  <CodeBlock
                    language="typescript"
                    filename="css_selectors.ts"
                    showLineNumbers
                    code={`const page = await session.page("https://example.com");

const headings = await page.css("h1");
const cards = await page.css(".product-card");`}
                  />
                ),
              },
            ]}
          />
        </section>

        {/* XPath Selectors */}
        <section id="xpath-selectors" className="scroll-mt-20">
          <h2 className="text-lg font-bold text-black dark:text-white mb-2">XPath Queries</h2>
          <p className="text-xs text-gray-500 dark:text-gray-400 mb-4 leading-relaxed">
            Perform complex structure navigation with element attribute searches, direct paths, or parent relative nodes.
          </p>
          <CodeBlock
            language="python"
            filename="xpath.py"
            showLineNumbers
            code={`# Match tables rows
rows = page.xpath("//table/tr")

# Select inputs with type rules
inputs = page.xpath("//input[@type='text' and @name='email']")`}
          />
        </section>

        {/* Text Anchor Selectors */}
        <section id="text-selectors" className="scroll-mt-20">
          <h2 className="text-lg font-bold text-black dark:text-white mb-2">Text Anchor (SIMD-Accelerated)</h2>
          <p className="text-xs text-gray-500 dark:text-gray-400 mb-4 leading-relaxed">
            Anchor elements based on literal content values. Excellent for extracting variable data located next to static titles (e.g. price tags or sku names).
          </p>
          <CodeBlock
            language="python"
            filename="text_anchor.py"
            showLineNumbers
            code={`# Find exact cell text
element = page.find_text("SKU Number:")

# Get the adjacent details element
detail = page.after_text("SKU Number:")`}
          />
        </section>

        {/* Self-Healing Auto-Match */}
        <section id="auto-match" className="scroll-mt-20">
          <h2 className="text-lg font-bold text-black dark:text-white mb-2">Auto-Match Self-Healing</h2>
          <p className="text-xs text-gray-500 dark:text-gray-400 mb-4 leading-relaxed">
            Crawlingo tracks element profiles (tag, attributes, text lengths, children profiles) to dynamically resolve CSS paths during a web layout modification. Try it live:
          </p>

          {/* Interactive Selector Match Simulator */}
          <SelectorSimulator />

          <CodeBlock
            language="python"
            filename="auto_heal.py"
            showLineNumbers
            code={`session = Session()
session.auto_match(True) # Activates self-healing heuristics

# Customize similarity scoring weights (e.g., prioritize text content)
session.auto_match_weights({
    "text": 3.0,
    "class": 1.0,
    "depth": 0.5
})

# Crawlingo compares current page structure with previously cached DOM fingerprints.
# Matches are recovered dynamically without editing scrapers code.
dataset = session.dataset("https://example.com/product/1")`}
          />
        </section>

        {/* Stealth wreq Fetcher */}
        <section id="stealthy-fetcher" className="scroll-mt-20">
          <h2 className="text-lg font-bold text-black dark:text-white mb-2">Stealthy Browser Impersonation</h2>
          <p className="text-xs text-gray-500 dark:text-gray-400 mb-4 leading-relaxed">
            Impersonates browser signatures (TLS, HTTP/2 frames, connection timings) natively inside Rust without browser instance memory footprint.
          </p>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 my-6">
            <HoverBorderCard className="p-5">
              <h4 className="text-xs font-semibold text-black dark:text-white mb-1">Rotated Fingerprints</h4>
              <p className="text-xs text-gray-500 dark:text-gray-400 leading-relaxed">Rotates JA3 signatures matching recent Chrome, Firefox, or Safari versions.</p>
            </HoverBorderCard>
            <HoverBorderCard className="p-5">
              <h4 className="text-xs font-semibold text-black dark:text-white mb-1">Governor Rate Limits</h4>
              <p className="text-xs text-gray-500 dark:text-gray-400 leading-relaxed">Protects endpoint loads with per-domain limits and connection reuse rules.</p>
            </HoverBorderCard>
          </div>
          <CodeBlock
            language="python"
            filename="stealth.py"
            code={`session.fetcher_tier("stealthy")
session.browser_profile("chrome")`}
          />
        </section>

        {/* Dataset Result exports */}
        <section id="dataset" className="scroll-mt-20">
          <h2 className="text-lg font-bold text-black dark:text-white mb-2">Multi-Format Exports</h2>
          <p className="text-xs text-gray-500 dark:text-gray-400 mb-4 leading-relaxed">
            Save extraction results into databases or pass them directly to analytical frameworks.
          </p>
          <CodeBlock
            language="python"
            filename="exports.py"
            code={`result = dataset.build()
result.to_json("data.json")
result.to_csv("data.csv")
result.to_parquet("data.parquet") # Native Arrow file
df = result.df()                  # Returns standard Pandas DataFrame`}
          />
        </section>

        {/* Watches */}
        <section id="watch" className="scroll-mt-20">
          <h2 className="text-lg font-bold text-black dark:text-white mb-2">Change Monitoring Watches</h2>
          <p className="text-xs text-gray-500 dark:text-gray-400 mb-4 leading-relaxed">
            Setup polling on elements to trigger callbacks when elements are added, deleted, or values (like price changes) drift.
          </p>
          <CodeBlock
            language="python"
            filename="watch.py"
            code={`watch = session.watch("https://example.com/item")
watch.field("price", ".price")
watch.interval(60) # Watch item every 60 seconds

watch.on_price_change(lambda ev: print(f"Price modified: {ev.old_value} -> {ev.new_value}"))
watch.run()`}
          />
        </section>
      </div>
    </div>
  );
}
