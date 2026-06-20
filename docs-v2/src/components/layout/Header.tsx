import { useState, useEffect, useRef } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { Search, Menu, X, Github, Sun, Moon, ArrowRight, FileText, Book, Code2 } from 'lucide-react';
import { Link, useLocation } from 'react-router-dom';
import { cn } from '@/lib/utils';
import EagleLogo from '@/components/ui/EagleLogo';

interface HeaderProps {
  onMenuToggle: () => void;
  isMobileMenuOpen: boolean;
}

export default function Header({ onMenuToggle, isMobileMenuOpen }: HeaderProps) {
  const location = useLocation();
  const isWaitlist = location.pathname === '/waitlist';
  const [scrolled, setScrolled] = useState(false);
  const [isSearchOpen, setIsSearchOpen] = useState(false);
  const [isDark, setIsDark] = useState(true);

  useEffect(() => {
    const handleScroll = () => setScrolled(window.scrollY > 10);
    window.addEventListener('scroll', handleScroll);
    return () => window.removeEventListener('scroll', handleScroll);
  }, []);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault();
        setIsSearchOpen((prev) => !prev);
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, []);

  useEffect(() => {
    if (isDark) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  }, [isDark]);

  return (
    <>
      <header
        className={cn(
          'fixed top-0 left-0 right-0 h-16 z-50 transition-all duration-300',
          scrolled
            ? 'bg-white/80 dark:bg-black/80 backdrop-blur-xl border-b border-gray-200 dark:border-white/10 shadow-sm'
            : 'bg-transparent'
        )}
      >
        <div className="flex items-center justify-between h-full px-4 lg:px-6 max-w-[1400px] mx-auto">
          <div className="flex items-center gap-4">
            {!isWaitlist && (
              <button
                onClick={onMenuToggle}
                className="lg:hidden p-2 -ml-2 text-gray-600 dark:text-gray-400 hover:text-black dark:hover:text-white transition-colors"
                aria-label="Toggle menu"
              >
                {isMobileMenuOpen ? <X size={20} /> : <Menu size={20} />}
              </button>
            )}

            <a href="/" className="flex items-center gap-3 group">
              <EagleLogo size="md" />
              <span className="font-bold text-lg text-black dark:text-white tracking-tight hidden sm:block">
                Crawlingo
              </span>
            </a>
          </div>

          <div className="hidden md:block flex-1 max-w-md mx-8">
            <button
              onClick={() => setIsSearchOpen(true)}
              className="w-full flex items-center gap-3 px-4 py-2.5 bg-gray-100 dark:bg-white/5 border border-gray-200 dark:border-white/10 rounded-xl text-gray-500 dark:text-gray-400 text-sm hover:bg-gray-200 dark:hover:bg-white/10 hover:border-gray-300 dark:hover:border-white/20 transition-all duration-200"
            >
              <Search size={16} className="text-gray-400 dark:text-gray-500" />
              <span className="flex-1 text-left">Search documentation...</span>
              <div className="flex items-center gap-1">
                <kbd className="text-[10px] font-mono bg-white dark:bg-gray-800 px-1.5 py-0.5 rounded border border-gray-200 dark:border-gray-700 text-gray-600 dark:text-gray-400">
                  ⌘
                </kbd>
                <kbd className="text-[10px] font-mono bg-white dark:bg-gray-800 px-1.5 py-0.5 rounded border border-gray-200 dark:border-gray-700 text-gray-600 dark:text-gray-400">
                  K
                </kbd>
              </div>
            </button>
          </div>

          <div className="flex items-center gap-2">
            {isWaitlist ? (
              <Link
                to="/"
                className="hidden sm:flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-gray-100 text-black dark:bg-white/10 dark:text-white text-xs font-semibold hover:opacity-90 transition-opacity mr-2 border border-gray-200 dark:border-white/10"
              >
                View Documentation
                <ArrowRight size={12} />
              </Link>
            ) : (
              <Link
                to="/waitlist"
                className="hidden sm:flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-black text-white dark:bg-white dark:text-black text-xs font-semibold hover:opacity-90 transition-opacity mr-2"
              >
                Join Waitlist
                <ArrowRight size={12} />
              </Link>
            )}

            <button
              onClick={() => setIsSearchOpen(true)}
              className="md:hidden p-2 text-gray-600 dark:text-gray-400 hover:text-black dark:hover:text-white transition-colors"
            >
              <Search size={18} />
            </button>

            <a
              href="https://github.com/crawlingo/crawlingo"
              target="_blank"
              rel="noopener noreferrer"
              className="p-2 text-gray-600 dark:text-gray-400 hover:text-black dark:hover:text-white transition-colors"
            >
              <Github size={18} />
            </a>

            <button
              onClick={() => setIsDark(!isDark)}
              className="p-2 text-gray-600 dark:text-gray-400 hover:text-black dark:hover:text-white transition-colors rounded-lg hover:bg-gray-100 dark:hover:bg-white/10"
            >
              {isDark ? <Sun size={18} /> : <Moon size={18} />}
            </button>
          </div>
        </div>
      </header>

      <SearchModal isOpen={isSearchOpen} onClose={() => setIsSearchOpen(false)} />
    </>
  );
}

function SearchModal({ isOpen, onClose }: { isOpen: boolean; onClose: () => void }) {
  const [query, setQuery] = useState('');
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (!isOpen) return;

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        onClose();
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [isOpen, onClose]);

  useEffect(() => {
    if (isOpen) {
      setQuery('');
      setTimeout(() => inputRef.current?.focus(), 50);
    }
  }, [isOpen]);

  const results = [
    { title: 'Introduction', path: '/', section: 'Getting Started', icon: <FileText size={16} /> },
    { title: 'Installation', path: '/getting-started', section: 'Getting Started', icon: <FileText size={16} /> },
    { title: 'Architecture', path: '/architecture', section: 'Core Concepts', icon: <Book size={16} /> },
    { title: 'Features', path: '/features', section: 'Features', icon: <Book size={16} /> },
    { title: 'API Reference', path: '/api-reference', section: 'Reference', icon: <Code2 size={16} /> },
    { title: 'Python SDK', path: '/sdk', section: 'SDKs', icon: <Code2 size={16} /> },
    { title: 'Node.js SDK', path: '/sdk', section: 'SDKs', icon: <Code2 size={16} /> },
    { title: 'Integrations', path: '/integrations', section: 'Integrations', icon: <Code2 size={16} /> },
    { title: 'Advanced Topics', path: '/advanced', section: 'Advanced', icon: <Book size={16} /> },
    { title: 'Troubleshooting', path: '/troubleshooting', section: 'Help', icon: <Book size={16} /> },
  ];

  const filtered = query
    ? results.filter(r =>
        r.title.toLowerCase().includes(query.toLowerCase()) ||
        r.section.toLowerCase().includes(query.toLowerCase())
      )
    : results;

  return (
    <AnimatePresence>
      {isOpen && (
        <div className="fixed inset-0 z-[9999] flex items-center justify-center">
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            transition={{ duration: 0.15 }}
            onClick={onClose}
            className="absolute inset-0 bg-black/50 backdrop-blur-sm"
          />
          <motion.div
            initial={{ opacity: 0, scale: 0.96 }}
            animate={{ opacity: 1, scale: 1 }}
            exit={{ opacity: 0, scale: 0.96 }}
            transition={{ duration: 0.15, ease: [0.16, 1, 0.3, 1] }}
            className="relative w-full max-w-2xl mx-4 max-h-[80vh] overflow-hidden rounded-2xl border border-gray-200 dark:border-white/10 bg-white dark:bg-gray-950 shadow-2xl"
          >
            <div className="flex items-center gap-3 px-4 border-b border-gray-200 dark:border-white/10">
              <Search size={16} className="text-gray-400 shrink-0" />
              <input
                ref={inputRef}
                type="text"
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                placeholder="Search documentation..."
                className="flex-1 py-3.5 bg-transparent text-black dark:text-white placeholder-gray-400 dark:placeholder-gray-500 outline-none text-sm"
              />
              <kbd className="text-[10px] font-mono text-gray-500 bg-gray-100 dark:bg-white/10 px-1.5 py-0.5 rounded border border-gray-200 dark:border-white/10">
                ESC
              </kbd>
            </div>
            <div className="max-h-[60vh] overflow-y-auto p-2">
              {filtered.length === 0 ? (
                <div className="text-center py-12 text-gray-500 dark:text-gray-400 text-sm">
                  No results found for &quot;{query}&quot;
                </div>
              ) : (
                <div className="space-y-0.5">
                  {filtered.map((result, i) => (
                    <a
                      key={i}
                      href={result.path}
                      onClick={onClose}
                      className="flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm hover:bg-gray-100 dark:hover:bg-white/5 transition-colors group"
                    >
                      <div className="w-8 h-8 rounded-lg bg-gray-100 dark:bg-white/5 flex items-center justify-center text-gray-500 dark:text-gray-400 group-hover:bg-black dark:group-hover:bg-white group-hover:text-white dark:group-hover:text-black transition-colors shrink-0">
                        {result.icon}
                      </div>
                      <div className="flex-1 min-w-0">
                        <div className="font-medium text-black dark:text-white">{result.title}</div>
                        <div className="text-[11px] text-gray-500 dark:text-gray-500">{result.section}</div>
                      </div>
                      <ArrowRight size={12} className="text-gray-400 opacity-0 group-hover:opacity-100 transition-opacity shrink-0" />
                    </a>
                  ))}
                </div>
              )}
            </div>
            <div className="px-4 py-2.5 border-t border-gray-200 dark:border-white/10 flex items-center gap-4 text-[11px] text-gray-500 dark:text-gray-500">
              <span className="flex items-center gap-1">
                <kbd className="px-1.5 py-0.5 bg-gray-100 dark:bg-white/10 rounded border border-gray-200 dark:border-white/10">↵</kbd>
                select
              </span>
              <span className="flex items-center gap-1">
                <kbd className="px-1.5 py-0.5 bg-gray-100 dark:bg-white/10 rounded border border-gray-200 dark:border-white/10">↑↓</kbd>
                navigate
              </span>
              <span className="flex items-center gap-1">
                <kbd className="px-1.5 py-0.5 bg-gray-100 dark:bg-white/10 rounded border border-gray-200 dark:border-white/10">esc</kbd>
                close
              </span>
            </div>
          </motion.div>
        </div>
      )}
    </AnimatePresence>
  );
}
