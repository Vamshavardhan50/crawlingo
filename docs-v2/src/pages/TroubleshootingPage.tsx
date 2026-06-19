import { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { ChevronDown, Search } from 'lucide-react';
import CodeBlock from '@/components/ui/CodeBlock';
import Callout from '@/components/ui/Callout';

interface FAQItem {
  question: string;
  answer: string;
  code?: string;
}

const faqSections: { title: string; items: FAQItem[] }[] = [
  {
    title: 'Installation failures',
    items: [
      {
        question: 'Fails with "No matching distribution found"',
        answer: 'Crawlingo requires Python 3.8+ or Node.js 16+. Check your version and ensure you have a compatible platform (Linux, macOS, or Windows x86_64).',
        code: `python --version # Check Python\nnode --version   # Check Node.js`,
      },
      {
        question: 'Rust compiler tools requested during installation',
        answer: 'You do NOT need Rust compiler chains. Pre-compiled binaries are shipped directly on PyPI/npm. Force a binary clean install:',
        code: `# Re-install Python SDK\npip install --force-reinstall crawlingo\n\n# Re-install Node.js SDK\nnpm install @crawlingo/sdk --force`,
      },
    ],
  },
  {
    title: 'Selector extraction issues',
    items: [
      {
        question: 'CSS selectors return empty arrays',
        answer: 'Verify selectors inside browser inspection tools. Crawlingo parses static elements list, so client-side dynamic frame tags (like canvas templates) might require wait conditions.',
      },
      {
        question: 'XPath selectors fail to resolve',
        answer: 'Standard child/sibling paths are fully supported. For dynamic index checks, use find_text/after_text instead.',
      },
    ],
  },
];

function FAQAccordion({ item }: { item: FAQItem }) {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <div className="border border-gray-200 dark:border-white/10 rounded-xl overflow-hidden bg-white dark:bg-zinc-950 transition-all hover:border-black/30 dark:hover:border-white/20">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="w-full flex items-center justify-between p-4 text-left transition-colors"
      >
        <span className="text-xs font-semibold text-black dark:text-white pr-4">{item.question}</span>
        <motion.div
          animate={{ rotate: isOpen ? 180 : 0 }}
          transition={{ duration: 0.2 }}
          className="shrink-0"
        >
          <ChevronDown size={14} className="text-gray-400" />
        </motion.div>
      </button>

      <AnimatePresence initial={false}>
        {isOpen && (
          <motion.div
            initial={{ height: 0, opacity: 0 }}
            animate={{ height: 'auto', opacity: 1 }}
            exit={{ height: 0, opacity: 0 }}
            transition={{ duration: 0.2, ease: [0.16, 1, 0.3, 1] }}
            className="overflow-hidden"
          >
            <div className="px-4 pb-4">
              <p className="text-xs text-gray-500 dark:text-gray-400 mb-3">{item.answer}</p>
              {item.code && (
                <CodeBlock code={item.code} language="bash" />
              )}
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}

export default function TroubleshootingPage() {
  const [searchQuery, setSearchQuery] = useState('');

  const filteredSections = faqSections.map((section) => ({
    ...section,
    items: section.items.filter(
      (item) =>
        item.question.toLowerCase().includes(searchQuery.toLowerCase()) ||
        item.answer.toLowerCase().includes(searchQuery.toLowerCase())
    ),
  })).filter((section) => section.items.length > 0);

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
          Troubleshooting FAQ
        </h1>
        <p className="text-base text-gray-600 dark:text-gray-400 max-w-2xl leading-relaxed">
          Quickly resolve common setup failures, selector issues, and rate limits problems.
        </p>
      </motion.div>

      {/* Search */}
      <div>
        <div className="flex items-center gap-3 px-4 py-2.5 bg-gray-50 dark:bg-white/5 border border-gray-200 dark:border-white/10 rounded-xl">
          <Search size={16} className="text-gray-400" />
          <input
            type="text"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            placeholder="Search questions..."
            className="flex-1 bg-transparent text-black dark:text-white placeholder-gray-400 dark:placeholder-gray-500 outline-none text-xs"
          />
        </div>
      </div>

      {/* FAQ list */}
      <div className="space-y-8">
        {filteredSections.map((section, i) => (
          <section key={i}>
            <h2 className="text-sm font-bold uppercase tracking-widest text-gray-400 mb-4">{section.title}</h2>
            <div className="space-y-3">
              {section.items.map((item, j) => (
                <FAQAccordion key={j} item={item} />
              ))}
            </div>
          </section>
        ))}
      </div>

      <Callout type="info">
        Still having problems? Join the Discord channel or open an issue on the GitHub repository.
      </Callout>
    </div>
  );
}
