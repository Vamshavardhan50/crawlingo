import { useState } from 'react';
import { Outlet, useLocation } from 'react-router-dom';
import { AnimatePresence, motion } from 'framer-motion';
import Header from './Header';
import Sidebar from './Sidebar';
import BackgroundGrid from '@/components/ui/BackgroundGrid';
import TracingScrollBeam from '@/components/ui/TracingScrollBeam';
import { cn } from '@/lib/utils';

export default function DocsLayout() {
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);
  const location = useLocation();
  const isWaitlist = location.pathname === '/waitlist';

  const content = (
    <AnimatePresence mode="wait">
      <motion.div
        key={location.pathname}
        initial={{ opacity: 0, y: 15 }}
        animate={{ opacity: 1, y: 0 }}
        exit={{ opacity: 0, y: -10 }}
        transition={{ duration: 0.3, ease: [0.16, 1, 0.3, 1] }}
      >
        <Outlet />
      </motion.div>
    </AnimatePresence>
  );

  return (
    <BackgroundGrid pattern="grid">
      <Header
        onMenuToggle={() => setIsMobileMenuOpen(!isMobileMenuOpen)}
        isMobileMenuOpen={isMobileMenuOpen}
      />
      {!isWaitlist && (
        <Sidebar
          isOpen={isMobileMenuOpen}
          onClose={() => setIsMobileMenuOpen(false)}
        />
      )}

      <main className={cn('pt-16', !isWaitlist && 'lg:pl-64')}>
        <div className={cn('mx-auto px-6 py-12 lg:px-8', isWaitlist ? 'max-w-5xl' : 'max-w-4xl')}>
          {isWaitlist ? (
            content
          ) : (
            <TracingScrollBeam>
              {content}
            </TracingScrollBeam>
          )}
        </div>
      </main>
    </BackgroundGrid>
  );
}

