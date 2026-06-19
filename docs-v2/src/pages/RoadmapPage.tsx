import { motion } from 'framer-motion';
import { Check, Clock, ArrowUpRight } from 'lucide-react';
import HoverBorderCard from '@/components/ui/HoverBorderCard';

interface RoadmapItem {
  title: string;
  description: string;
  status: 'done' | 'in-progress' | 'planned';
}

interface RoadmapSection {
  quarter: string;
  items: RoadmapItem[];
}

const roadmap: RoadmapSection[] = [
  {
    quarter: 'Q2 2026',
    items: [
      { title: 'Rust Core Engine', description: 'wreq HTTP client, lol_html parser, selector engine', status: 'done' },
      { title: 'Python SDK (PyO3)', description: 'Session, Page, Element, ElementCollection, Dataset, Crawl, Watch', status: 'done' },
      { title: 'Node.js SDK (napi-rs)', description: 'Same API surface as Python with TypeScript definitions', status: 'done' },
      { title: '4 Selector Types', description: 'CSS, XPath, Text Anchor, Regex with DashMap caching', status: 'done' },
      { title: 'Auto-Match Self-Healing', description: 'DOM fingerprinting with Jaro-Winkler similarity', status: 'done' },
      { title: 'Stealthy Fetcher', description: 'Chrome/Firefox/Safari impersonation via wreq', status: 'done' },
      { title: 'Change Monitoring', description: 'Watch with on_change, on_price_change, on_stock_change callbacks', status: 'done' },
      { title: 'MCP Server', description: 'fetch_page, extract_data, crawl_site tools', status: 'done' },
    ],
  },
  {
    quarter: 'Q3 2026',
    items: [
      { title: 'Enhanced Auto-Match', description: 'Improve similarity scoring weights and recovery accuracy', status: 'done' },
      { title: 'Proxy Pool Integration', description: 'Built-in proxy rotation with provider APIs', status: 'done' },
      { title: 'Scheduled Crawling', description: 'Cron-based scheduling for recurring crawls', status: 'done' },
      { title: 'Webhook Delivery', description: 'Send crawl results to webhooks in real-time', status: 'done' },
    ],
  },
  {
    quarter: 'Q4 2026',
    items: [
      { title: 'Dashboard UI', description: 'Web interface for monitoring crawls and watches', status: 'planned' },
      { title: 'Cloud Deployment', description: 'Managed cloud service for scheduled scraping', status: 'planned' },
      { title: 'JavaScript Rendering', description: 'Playwright integration for SPA support', status: 'planned' },
    ],
  },
];

const statusConfig = {
  done: { icon: Check, label: 'Completed', color: 'text-black bg-zinc-100 dark:text-white dark:bg-zinc-800', dot: 'bg-black dark:bg-white' },
  'in-progress': { icon: Clock, label: 'In Progress', color: 'text-black bg-zinc-100 dark:text-white dark:bg-zinc-800', dot: 'bg-black dark:bg-white' },
  planned: { icon: Clock, label: 'Planned', color: 'text-gray-400 bg-gray-50 dark:text-gray-500 dark:bg-white/5', dot: 'bg-gray-300 dark:bg-zinc-800' },
};

export default function RoadmapPage() {
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
          Roadmap
        </h1>
        <p className="text-base text-gray-600 dark:text-gray-400 max-w-2xl leading-relaxed">
          Features and milestones we are working on to expand the Crawlingo scraping framework.
        </p>
      </motion.div>

      {/* Target milestones */}
      <div className="space-y-12">
        {roadmap.map((section, idx) => (
          <div key={idx}>
            <div className="flex items-center gap-3 mb-6">
              <h2 className="text-sm font-bold text-black dark:text-white uppercase tracking-widest">{section.quarter}</h2>
              <div className="flex-1 h-px bg-gray-200 dark:bg-white/10" />
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {section.items.map((item, itemIdx) => {
                const config = statusConfig[item.status];
                const Icon = config.icon;
                return (
                  <HoverBorderCard key={itemIdx} className="p-5">
                    <div className="flex items-start justify-between gap-3">
                      <div>
                        <h4 className="text-xs font-semibold text-black dark:text-white mb-1">{item.title}</h4>
                        <p className="text-[11px] text-gray-500 dark:text-gray-400 leading-relaxed">{item.description}</p>
                      </div>
                      <span className={`text-[9px] font-mono font-semibold px-2 py-0.5 rounded flex items-center gap-1 shrink-0 ${config.color}`}>
                        <Icon size={8} />
                        {config.label}
                      </span>
                    </div>
                  </HoverBorderCard>
                );
              })}
            </div>
          </div>
        ))}
      </div>

      {/* Feedback Section */}
      <HoverBorderCard className="p-6">
        <div className="flex flex-col sm:flex-row items-center justify-between gap-4">
          <div>
            <h3 className="text-sm font-semibold text-black dark:text-white">Have feedback?</h3>
            <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">Suggest features or ask questions on our GitHub discussions page.</p>
          </div>
          <a
            href="https://github.com/crawlingo/crawlingo/discussions"
            target="_blank"
            rel="noopener noreferrer"
            className="btn-secondary py-2 px-4 text-xs font-semibold flex items-center gap-1.5"
          >
            Join discussions
            <ArrowUpRight size={14} />
          </a>
        </div>
      </HoverBorderCard>
    </div>
  );
}
