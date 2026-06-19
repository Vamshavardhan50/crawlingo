import { useState, useEffect } from 'react';
import { cn } from '@/lib/utils';

interface TOCItem {
  id: string;
  title: string;
  level: number;
}

interface TableOfContentsProps {
  items: TOCItem[];
}

// Note: This component is currently unused. Pages render content in a single
// centered column. Kept here for future reference / re-enablement.
export default function TableOfContents({ items }: TableOfContentsProps) {
  const [activeId, setActiveId] = useState('');

  useEffect(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            setActiveId(entry.target.id);
          }
        });
      },
      { rootMargin: '-80px 0px -80% 0px' }
    );

    items.forEach(({ id }) => {
      const element = document.getElementById(id);
      if (element) observer.observe(element);
    });

    return () => observer.disconnect();
  }, [items]);

  if (items.length === 0) return null;

  return (
    <aside className="hidden xl:block w-56 shrink-0">
      <div className="sticky top-20 pl-6 border-l border-gray-200 dark:border-white/10">
        <h4 className="text-[11px] font-semibold text-gray-500 dark:text-gray-500 uppercase tracking-wider mb-3 pl-3">
          On this page
        </h4>
        <nav className="space-y-1">
          {items.map((item) => (
            <a
              key={item.id}
              href={`#${item.id}`}
              className={cn(
                'block text-[13px] py-1 transition-colors hover:text-black dark:hover:text-white',
                item.level === 3 && 'pl-6',
                item.level === 2 && 'pl-3',
                activeId === item.id
                  ? 'text-black dark:text-white font-medium border-l-2 border-black dark:border-white -ml-px pl-3 rounded-r-md'
                  : 'text-gray-500 dark:text-gray-400'
              )}
              onClick={(e) => {
                e.preventDefault();
                document.getElementById(item.id)?.scrollIntoView({ behavior: 'smooth' });
              }}
            >
              {item.title}
            </a>
          ))}
        </nav>
      </div>
    </aside>
  );
}
