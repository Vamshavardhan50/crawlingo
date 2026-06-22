import { motion } from 'framer-motion';
import { ArrowUpRight } from 'lucide-react';
import HoverBorderCard from '@/components/ui/HoverBorderCard';

const versions = [
  {
    version: '1.3.0',
    date: '2026-06-19',
    tag: 'stable',
    changes: [
      {
        type: 'added',
        items: [
          'Enhanced Auto-Match: Customizable similarity scoring weights (text, class, depth, tags, attributes) for finer healing control.',
          'Built-in IP Proxy Pools & rotation with provider URL loading supports directly on Session level.',
          'Scheduled Crawling: Background cron/interval runner for recurring crawl tasks.',
          'Real-time Webhook Delivery: Stream extracted JSON items dynamically to external API endpoints during crawls.',
          'Exposed Python & napi-rs Node.js bindings mapping the new session and crawler structures.',
        ],
      },
    ],
  },
  {
    version: '1.2.0',
    date: '2026-06-18',
    tag: 'stable',
    changes: [
      {
        type: 'added',
        items: [
          'Session variables with timeout, headers, cookies, and rate_limits configuration.',
          'Page selectors with css(), xpath(), find_text(), after_text(), before_text() interfaces.',
          'Auto-match self-healing capabilities utilizing Jaro-Winkler scores stored in sled databases.',
          'Direct Arrow table parsing with parquet/json exports and Pandas df() callbacks.',
          'Stealth browser emulation signatures using wreq bindings.',
        ],
      },
    ],
  },
];

export default function ChangelogPage() {
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
          Changelog
        </h1>
        <p className="text-base text-gray-600 dark:text-gray-400 max-w-2xl leading-relaxed">
          Release logs and version history details for Crawlingo framework components.
        </p>
      </motion.div>

      {/* History */}
      <div className="space-y-8">
        {versions.map((ver, idx) => (
          <HoverBorderCard key={idx} className="p-6">
            <div className="flex items-center gap-3 mb-4">
              <h2 className="text-base font-bold text-black dark:text-white">v{ver.version}</h2>
              <span className="text-[10px] font-mono font-semibold uppercase px-2 py-0.5 bg-black text-white dark:bg-white dark:text-black rounded">
                {ver.tag}
              </span>
              <span className="text-xs text-gray-400">{ver.date}</span>
            </div>

            <div className="space-y-4">
              {ver.changes.map((change, cIdx) => (
                <div key={cIdx}>
                  <span className="text-[10px] font-mono font-semibold text-gray-400 uppercase tracking-widest block mb-2">{change.type}</span>
                  <ul className="space-y-2">
                    {change.items.map((item, itemIdx) => (
                      <li key={itemIdx} className="text-xs text-gray-500 dark:text-gray-400 flex items-start gap-2">
                        <span className="text-gray-400 mt-1">•</span>
                        <span>{item}</span>
                      </li>
                    ))}
                  </ul>
                </div>
              ))}
            </div>
          </HoverBorderCard>
        ))}
      </div>

      {/* GitHub Section */}
      <HoverBorderCard className="p-6">
        <div className="flex flex-col sm:flex-row items-center justify-between gap-4">
          <div>
            <h3 className="text-sm font-semibold text-black dark:text-white">Stay Tuned</h3>
            <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">Watch releases directly on GitHub for real-time update notifications.</p>
          </div>
          <a
            href="https://github.com/Vamshavardhan50/crawlingo/releases"
            target="_blank"
            rel="noopener noreferrer"
            className="btn-secondary py-2 px-4 text-xs font-semibold flex items-center gap-1.5"
          >
            View Releases
            <ArrowUpRight size={14} />
          </a>
        </div>
      </HoverBorderCard>
    </div>
  );
}
