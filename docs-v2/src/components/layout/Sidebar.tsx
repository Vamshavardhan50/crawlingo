import { useState } from 'react';
import { Link, useLocation } from 'react-router-dom';
import { motion, AnimatePresence } from 'framer-motion';
import { ChevronRight, BookOpen, Layers, Zap, Code2, Puzzle, Wrench, HelpCircle, FileText, Map, ArrowRight, Play } from 'lucide-react';
import { cn } from '@/lib/utils';

interface SidebarProps {
  isOpen: boolean;
  onClose: () => void;
}

interface NavItem {
  title: string;
  path: string;
  icon?: React.ReactNode;
  badge?: string;
}

interface NavGroup {
  title: string;
  items: NavItem[];
}

const navigation: NavGroup[] = [
  {
    title: 'Getting Started',
    items: [
      { title: 'Introduction', path: '/', icon: <BookOpen size={16} /> },
      { title: 'Quick Start', path: '/getting-started', icon: <Zap size={16} /> },
      { title: 'Interactive Demo', path: '/demo', icon: <Play size={16} />, badge: 'Watch' },
    ],
  },
  {
    title: 'Core',
    items: [
      { title: 'Architecture', path: '/architecture', icon: <Layers size={16} /> },
      { title: 'Features', path: '/features', icon: <Zap size={16} />, badge: 'New' },
    ],
  },
  {
    title: 'Reference',
    items: [
      { title: 'API Reference', path: '/api-reference', icon: <Code2 size={16} /> },
      { title: 'SDK Documentation', path: '/sdk', icon: <Code2 size={16} /> },
      { title: 'Integrations', path: '/integrations', icon: <Puzzle size={16} /> },
    ],
  },
  {
    title: 'More',
    items: [
      { title: 'Advanced Topics', path: '/advanced', icon: <Wrench size={16} /> },
      { title: 'Troubleshooting', path: '/troubleshooting', icon: <HelpCircle size={16} /> },
      { title: 'Changelog', path: '/changelog', icon: <FileText size={16} /> },
      { title: 'Roadmap', path: '/roadmap', icon: <Map size={16} /> },
    ],
  },
];

function NavGroupComponent({ group }: { group: NavGroup }) {
  const [isOpen, setIsOpen] = useState(true);
  const location = useLocation();

  return (
    <div className="mb-6">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="w-full flex items-center justify-between px-3 py-2 text-[11px] font-semibold text-gray-500 dark:text-gray-500 uppercase tracking-wider hover:text-black dark:hover:text-white transition-colors"
      >
        <span>{group.title}</span>
        <motion.div
          animate={{ rotate: isOpen ? 90 : 0 }}
          transition={{ duration: 0.2 }}
        >
          <ChevronRight size={12} />
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
            <div className="py-1 space-y-0.5">
              {group.items.map((item) => {
                const isActive = location.pathname === item.path;
                return (
                  <Link
                    key={item.path}
                    to={item.path}
                    className={cn(
                      'flex items-center gap-3 px-3 py-2 rounded-lg text-[13px] transition-all duration-200 group',
                      isActive
                        ? 'bg-black dark:bg-white text-white dark:text-black font-medium'
                        : 'text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-white/5 hover:text-black dark:hover:text-white'
                    )}
                  >
                    <span className={cn(
                      'transition-colors',
                      isActive ? 'text-white dark:text-black' : 'text-gray-400 dark:text-gray-500 group-hover:text-black dark:group-hover:text-white'
                    )}>
                      {item.icon}
                    </span>
                    <span>{item.title}</span>
                    {item.badge && (
                      <span className="ml-auto text-[10px] font-semibold px-2 py-0.5 rounded-full bg-black dark:bg-white text-white dark:text-black">
                        {item.badge}
                      </span>
                    )}
                    {isActive && (
                      <ArrowRight size={12} className="ml-auto" />
                    )}
                  </Link>
                );
              })}
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}

export default function Sidebar({ isOpen, onClose }: SidebarProps) {
  return (
    <>
      <AnimatePresence>
        {isOpen && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            onClick={onClose}
            className="fixed inset-0 bg-black/50 backdrop-blur-sm z-40 lg:hidden"
          />
        )}
      </AnimatePresence>

      <aside
        className={cn(
          'fixed top-16 left-0 bottom-0 w-64 bg-white dark:bg-black border-r border-gray-200 dark:border-white/10 overflow-y-auto z-40 transition-transform duration-300 lg:translate-x-0',
          isOpen ? 'translate-x-0' : '-translate-x-full'
        )}
      >
        <nav className="p-4 space-y-2">
          {navigation.map((group) => (
            <NavGroupComponent key={group.title} group={group} />
          ))}
        </nav>

        <div className="p-4 border-t border-gray-200 dark:border-white/10">
          <div className="p-4 rounded-xl bg-gray-50 dark:bg-white/5">
            <p className="text-xs font-medium text-black dark:text-white mb-2">Need help?</p>
            <p className="text-[11px] text-gray-500 dark:text-gray-400 mb-3">
              Join our community or check the docs.
            </p>
            <div className="flex gap-2">
              <a
                href="https://discord.gg/crawlingo"
                target="_blank"
                rel="noopener noreferrer"
                className="flex-1 text-center text-[11px] font-medium px-3 py-1.5 rounded-lg bg-black dark:bg-white text-white dark:text-black hover:opacity-90 transition-opacity"
              >
                Discord
              </a>
              <a
                href="https://github.com/crawlingo/crawlingo"
                target="_blank"
                rel="noopener noreferrer"
                className="flex-1 text-center text-[11px] font-medium px-3 py-1.5 rounded-lg border border-gray-200 dark:border-white/10 text-black dark:text-white hover:bg-gray-100 dark:hover:bg-white/5 transition-colors"
              >
                GitHub
              </a>
            </div>
          </div>
        </div>
      </aside>
    </>
  );
}
