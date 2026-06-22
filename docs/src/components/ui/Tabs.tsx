import { useState } from 'react';
import { motion } from 'framer-motion';
import { cn } from '@/lib/utils';

interface Tab {
  id: string;
  label: string;
  content: React.ReactNode;
}

interface TabsProps {
  tabs: Tab[];
  defaultTab?: string;
  className?: string;
}

export default function Tabs({ tabs, defaultTab, className }: TabsProps) {
  const [activeTab, setActiveTab] = useState(defaultTab || tabs[0]?.id);

  return (
    <div className={cn('my-6', className)}>
      <div className="flex gap-1 p-1 bg-gray-100 dark:bg-white/5 rounded-xl border border-gray-200 dark:border-white/10 mb-4">
        {tabs.map((tab) => (
          <button
            key={tab.id}
            onClick={() => setActiveTab(tab.id)}
            className={cn(
              'relative px-4 py-2 text-sm font-medium rounded-lg transition-all duration-200',
              activeTab === tab.id
                ? 'text-white dark:text-black'
                : 'text-gray-600 dark:text-gray-400 hover:text-black dark:hover:text-white'
            )}
          >
            {activeTab === tab.id && (
              <motion.div
                layoutId="activeTab"
                className="absolute inset-0 bg-black dark:bg-white rounded-lg"
                transition={{ duration: 0.2, ease: [0.16, 1, 0.3, 1] }}
              />
            )}
            <span className="relative z-10">{tab.label}</span>
          </button>
        ))}
      </div>

      <motion.div
        key={activeTab}
        initial={{ opacity: 0, y: 10 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.3, ease: [0.16, 1, 0.3, 1] }}
      >
        {tabs.find((t) => t.id === activeTab)?.content}
      </motion.div>
    </div>
  );
}
