import { useState, useEffect } from 'react';
import { Github, Sun, Moon, ArrowRight } from 'lucide-react';
import { cn } from '@/lib/utils';
import EagleLogo from '@/components/ui/EagleLogo';

export default function Header() {
  const [scrolled, setScrolled] = useState(false);
  const [isDark, setIsDark] = useState(true);

  useEffect(() => {
    const handleScroll = () => setScrolled(window.scrollY > 10);
    window.addEventListener('scroll', handleScroll);
    return () => window.removeEventListener('scroll', handleScroll);
  }, []);

  useEffect(() => {
    if (isDark) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  }, [isDark]);

  return (
    <header
      className={cn(
        'fixed top-0 left-0 right-0 h-16 z-50 transition-all duration-300',
        scrolled
          ? 'bg-white/80 dark:bg-black/80 backdrop-blur-xl border-b border-zinc-200 dark:border-white/10 shadow-sm'
          : 'bg-transparent'
      )}
    >
      <div className="flex items-center justify-between h-full px-4 lg:px-6 max-w-6xl mx-auto">
        <a href="/" className="flex items-center gap-3 group">
          <EagleLogo size="md" variant="full" />
        </a>

        <div className="flex items-center gap-3">
          <a
            href="https://crawlingo.vercel.app/docs/"
            className="flex items-center gap-1.5 px-3.5 py-1.5 rounded-xl border border-zinc-200 dark:border-white/10 text-xs font-semibold hover:bg-zinc-50 dark:hover:bg-white/5 transition-all text-zinc-700 dark:text-zinc-300"
          >
            Docs
            <ArrowRight size={12} />
          </a>

          <a
            href="https://github.com/Vamshavardhan50/crawlingo"
            target="_blank"
            rel="noopener noreferrer"
            className="p-2 text-zinc-600 dark:text-zinc-400 hover:text-black dark:hover:text-white transition-colors rounded-xl hover:bg-zinc-100 dark:hover:bg-white/5"
            aria-label="GitHub Repository"
          >
            <Github size={18} />
          </a>

          <button
            onClick={() => setIsDark(!isDark)}
            className="p-2 text-zinc-600 dark:text-zinc-400 hover:text-black dark:hover:text-white transition-colors rounded-xl hover:bg-zinc-100 dark:hover:bg-white/5"
            aria-label="Toggle Theme"
          >
            {isDark ? <Sun size={18} /> : <Moon size={18} />}
          </button>
        </div>
      </div>
    </header>
  );
}
