"use client";

import React from 'react';
import { cn } from '@/lib/utils';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { Moon, Sun, Menu, X, Search, GitBranch, BookOpen, Terminal } from 'lucide-react';
import { useTheme } from 'next-themes';

export function Layout({ children }: { children: React.ReactNode }) {
  const [sidebarOpen, setSidebarOpen] = React.useState(false);
  const { theme, setTheme } = useTheme();
  const pathname = usePathname();

  return (
    <div className="relative min-h-screen bg-background text-foreground antialiased selection:bg-indigo-500 selection:text-white">
      {/* Mobile sidebar backdrop */}
      {sidebarOpen && (
        <div 
          className="fixed inset-0 z-40 bg-black/50 backdrop-blur-sm transition-opacity duration-300"
          onClick={() => setSidebarOpen(false)}
        />
      )}

      {/* Sidebar navigation */}
      <aside
        className={cn(
          "fixed inset-y-0 left-0 z-50 w-72 transform bg-card/95 backdrop-blur-xl border-r border-border/50 transition-transform duration-300 ease-in-out",
          "lg:translate-x-0 lg:static lg:inset-auto lg:h-screen",
          sidebarOpen ? "translate-x-0" : "-translate-x-full"
        )}
      >
        <div className="flex h-full flex-col overflow-hidden">
          {/* Logo & Brand */}
          <div className="flex h-16 items-center gap-3 border-b border-border/50 px-6">
            <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-gradient-to-br from-indigo-500 to-purple-600">
              <Terminal className="h-4 w-4 text-white" />
            </div>
            <div className="flex flex-col">
              <span className="text-lg font-bold bg-gradient-to-r from-indigo-400 to-purple-400 bg-clip-text text-transparent">
                Crawlingo
              </span>
              <span className="text-xs text-muted-foreground">Web Scraping Framework</span>
            </div>
          </div>

          {/* Search */}
          <div className="p-4 border-b border-border/50">
            <div className="relative">
              <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
              <input
                type="text"
                placeholder="Search documentation..."
                className="w-full rounded-lg border border-border bg-background/50 py-2 pl-10 pr-4 text-sm transition-colors focus:border-indigo-500 focus:outline-none focus:ring-1 focus:ring-indigo-500"
              />
            </div>
          </div>

          {/* Navigation Items */}
          <nav className="flex-1 overflow-y-auto p-4">
            {NAV_ITEMS.map((section) => (
              <div key={section.title} className="mb-6">
                <h3 className="mb-2 px-3 text-xs font-semibold uppercase tracking-wider text-muted-foreground">
                  {section.title}
                </h3>
                <ul className="space-y-1">
                  {section.items.map((item) => {
                    const isActive = pathname === item.path;
                    return (
                      <li key={item.path}>
                        <Link
                          href={item.path}
                          className={cn(
                            "flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-all duration-200",
                            isActive
                              ? "bg-indigo-500/10 text-indigo-400 border-l-2 border-indigo-500"
                              : "text-muted-foreground hover:bg-muted/50 hover:text-foreground"
                          )}
                        >
                          <item.icon className="h-4 w-4" />
                          {item.label}
                        </Link>
                      </li>
                    );
                  })}
                </ul>
              </div>
            ))}

            {/* Version selector */}
            <div className="mt-8 border-t border-border/50 pt-4">
              <div className="flex items-center justify-between px-3">
                <GitBranch className="h-4 w-4 text-muted-foreground" />
                <select className="ml-2 rounded border border-border bg-background/50 px-2 py-1 text-xs">
                  <option>v0.1.0</option>
                  <option>v0.0.1</option>
                </select>
              </div>
            </div>

            {/* Theme toggle */}
            <div className="mt-4 px-3">
              <button
                onClick={() => setTheme(theme === 'dark' ? 'light' : 'dark')}
                className="flex w-full items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium text-muted-foreground transition-colors hover:bg-muted/50 hover:text-foreground"
              >
                {theme === 'dark' ? (
                  <Moon className="h-4 w-4" />
                ) : (
                  <Sun className="h-4 w-4" />
                )}
                Theme: {theme === 'dark' ? 'Dark' : 'Light'}
              </button>
            </div>
          </nav>

          {/* Mobile close button */}
          <div className="border-t border-border/50 p-4 lg:hidden">
            <button
              onClick={() => setSidebarOpen(false)}
              className="flex w-full items-center justify-center gap-2 rounded-lg border border-border bg-background/50 py-2 text-sm font-medium transition-colors hover:bg-muted/50"
            >
              <X className="h-4 w-4" />
              Close Menu
            </button>
          </div>
        </div>
      </aside>

      {/* Main content */}
      <div className="lg:ml-72">
        {/* Top bar */}
        <header className="sticky top-0 z-30 flex h-16 items-center gap-4 border-b border-border/50 bg-background/95 backdrop-blur-xl px-6 lg:hidden">
          <button
            onClick={() => setSidebarOpen(true)}
            className="inline-flex items-center justify-center rounded-lg border border-border bg-background p-2 transition-colors hover:bg-muted/50"
          >
            <Menu className="h-4 w-4" />
          </button>
          <div className="flex-1">
            <h1 className="text-lg font-semibold">Documentation</h1>
          </div>
        </header>

        {/* Page content */}
        <main className="min-w-0 flex-1">
          <div className="mx-auto max-w-4xl px-6 py-8 lg:px-12">
            {children}
          </div>
        </main>

        {/* Footer */}
        <footer className="border-t border-border/50 bg-card/30 py-12">
          <div className="mx-auto max-w-4xl px-6 lg:px-12">
            <div className="flex flex-col items-center justify-between gap-6 md:flex-row">
              <div className="flex items-center gap-3">
                <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-gradient-to-br from-indigo-500 to-purple-600">
                  <Terminal className="h-4 w-4 text-white" />
                </div>
                <div>
                  <span className="font-bold">Crawlingo</span>
                  <p className="text-sm text-muted-foreground">Effortless Self-Healing Web Scraping</p>
                </div>
              </div>
              <div className="flex gap-8 text-sm">
                <a href="#" className="text-muted-foreground hover:text-foreground">GitHub</a>
                <a href="#" className="text-muted-foreground hover:text-foreground">Twitter</a>
                <a href="#" className="text-muted-foreground hover:text-foreground">Discord</a>
                <a href="#" className="text-muted-foreground hover:text-foreground">API Reference</a>
              </div>
              <div className="text-sm text-muted-foreground">
                © 2024 Crawlingo. MIT License.
              </div>
            </div>
          </div>
        </footer>
      </div>
    </div>
  );
}

const NAV_ITEMS = [
  {
    title: "Getting Started",
    items: [
      { label: "Quick Start", path: "/quick-start", icon: BookOpen },
      { label: "Installation", path: "/installation", icon: BookOpen },
    ],
  },
  {
    title: "Features",
    items: [
      { label: "Self-Healing", path: "/features/auto-match", icon: BookOpen },
      { label: "Stealth Browsing", path: "/features/stealth", icon: BookOpen },
      { label: "SIMD Text Anchors", path: "/features/text-anchors", icon: BookOpen },
      { label: "Change Monitors", path: "/features/watches", icon: BookOpen },
      { label: "Dataset Export", path: "/features/datasets", icon: BookOpen },
    ],
  },
  {
    title: "SDKs",
    items: [
      { label: "Python", path: "/sdk/python", icon: BookOpen },
      { label: "Node.js", path: "/sdk/nodejs", icon: BookOpen },
      { label: "Rust", path: "/sdk/rust", icon: BookOpen },
    ],
  },
  {
    title: "API Reference",
    items: [
      { label: "Tools", path: "/tools/fetch-page", icon: Terminal },
      { label: "Sessions", path: "/api/session", icon: Terminal },
      { label: "Crawling", path: "/api/crawl", icon: Terminal },
      { label: "Changes", path: "/api/changes", icon: Terminal },
    ],
  },
  {
    title: "Resources",
    items: [
      { label: "Changelog", path: "/changelog", icon: BookOpen },
      { label: "Roadmap", path: "/roadmap", icon: BookOpen },
      { label: "GitHub", path: "https://github.com/Vamshavardhan50/crawlingo", icon: BookOpen },
    ],
  },
];
