import React from 'react';
import Link from 'next/link';
import { Card } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { FeatureCard } from '@/components/feature-card';
import { CodeBlock, TerminalExample } from '@/components/ui-card';
import { Rocket, Shield, GitBranch, Activity, Database, Search, Cpu, Globe, FileText, Layers, Webhook, Settings, Key, Terminal } from 'lucide-react';

export default function HomePage() {
  return (
    <div>
      {/* Hero Section */}
      <section className="relative py-20 md:py-32 overflow-hidden">
        <div className="absolute inset-0 bg-gradient-to-br from-indigo-900/20 via-purple-900/20 to-blue-900/20" />
        <div className="absolute inset-0 bg-grid-white/[0.02] bg-[size:50px_50px]" />
        
        <div className="relative mx-auto max-w-7xl px-6 lg:px-8">
          <div className="text-center">
            <h1 className="text-4xl font-bold tracking-tight text-foreground sm:text-6xl lg:text-7xl">
              Effortless Self-Healing
              <span className="bg-gradient-to-r from-indigo-400 via-purple-400 to-blue-400 bg-clip-text text-transparent">
                {' '}Web Scraping
              </span>
            </h1>
            <p className="mx-auto mt-6 max-w-2xl text-lg leading-8 text-muted-foreground">
              Crawlingo is an adaptive, high-performance web crawling and data extraction framework designed to survive website structure updates. By wrapping a compiled Rust core in developer-first Python and JavaScript APIs, Crawlingo provides cross-language, zero-copy, self-healing selectors under the hood.
            </p>
            
            {/* Quick Actions */}
            <div className="mt-10 flex flex-col items-center justify-center gap-4 sm:flex-row">
              <Link href="/quick-start">
                <Button size="lg" className="bg-indigo-600 hover:bg-indigo-700 h-12 px-8">
                  <Rocket className="mr-2 h-5 w-5" />
                  Get Started
                </Button>
              </Link>
              <Link href="/features/auto-match">
                <Button size="lg" variant="outline" className="h-12 px-8">
                  <Shield className="mr-2 h-5 w-5" />
                  See Features
                </Button>
              </Link>
            </div>
            
            {/* Version badge */}
            <div className="mt-8 flex items-center justify-center">
              <div className="flex items-center gap-2 rounded-full border border-indigo-500/30 bg-indigo-500/10 px-4 py-2 backdrop-blur-sm">
                <div className="h-2 w-2 rounded-full bg-green-500 animate-pulse" />
                <span className="text-sm font-medium">v0.1.0 - Available now</span>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* Feature Grid */}
      <section className="py-20 relative">
        <div className="mx-auto max-w-7xl px-6 lg:px-8">
          <div className="mx-auto max-w-2xl text-center">
            <h2 className="text-3xl font-bold tracking-tight sm:text-4xl">
              Why Choose Crawlingo?
            </h2>
            <p className="mt-4 text-lg text-muted-foreground">
              Built from the ground up with developer experience in mind.
            </p>
          </div>
          
          <div className="mt-16 grid gap-8 sm:grid-cols-2 lg:grid-cols-3">
            <FeatureCard
              icon={<Shield className="h-6 w-6 text-white" />}
              title="Self-Healing"
              description="Tracks layout changes and repairs selector mappings automatically when elements drift, ensuring your scraping pipeline never breaks."
              gradient={true}
            />
            <FeatureCard
              icon={<Globe className="h-6 w-6 text-white" />}
              title="Stealth Browsing"
              description="Bypasses advanced web verification (Cloudflare Turnstile, etc.) with minimal footprint using JA3/TLS handshake rotation."
            />
            <FeatureCard
              icon={<Cpu className="h-6 w-6 text-white" />}
              title="SIMD-Accelerated"
              description="Parallel CPU vectors for searching text occurrences and neighbors, enabling simple table extraction when classes are missing."
            />
            <FeatureCard
              icon={<GitBranch className="h-6 w-6 text-white" />}
              title="Proxy Rotation"
              description="Round-robin cycling using static pools or remote proxy list provider APIs automatically."
              gradient={true}
            />
            <FeatureCard
              icon={<Activity className="h-6 w-6 text-white" />}
              title="Change Monitoring"
              description="Tracks target DOM nodes and instantly publishes callbacks/webhooks when price quotes, stock changes, or layout modifications are discovered."
            />
            <FeatureCard
              icon={<Database className="h-6 w-6 text-white" />}
              title="Zero-Copy Core"
              description="Compiled Rust core with memory mappings shared across language SDKs for maximum performance."
              gradient={true}
            />
          </div>
        </div>
      </section>

      {/* Quick Start Example */}
      <section className="py-20 relative">
        <div className="mx-auto max-w-7xl px-6 lg:px-8">
          <div className="grid gap-12 lg:grid-cols-2 lg:gap-16 items-center">
            <div>
              <h2 className="text-3xl font-bold tracking-tight sm:text-4xl">
                Get Scraping in Minutes
              </h2>
              <p className="mt-4 text-lg text-muted-foreground">
                Start building with Crawlingo today. No complex configuration required.
              </p>
              
              <div className="mt-8 space-y-6">
                <div className="flex gap-4">
                  <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-indigo-500/10 text-indigo-400">
                    <Terminal className="h-4 w-4" />
                  </div>
                  <div>
                    <h3 className="font-semibold">Quick Start</h3>
                    <p className="text-sm text-muted-foreground">Get up and running in under 2 minutes</p>
                  </div>
                </div>
                
                <div className="flex gap-4">
                  <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-purple-500/10 text-purple-400">
                    <FileText className="h-4 w-4" />
                  </div>
                  <div>
                    <h3 className="font-semibold">Dataset Export</h3>
                    <p className="text-sm text-muted-foreground">Export to JSON, CSV, Parquet, and more</p>
                  </div>
                </div>
                
                <div className="flex gap-4">
                  <div className="flex h-8 w-8 items-center justify-center rounded-lg bg-blue-500/10 text-blue-400">
                    <Layers className="h-4 w-4" />
                  </div>
                  <div>
                    <h3 className="font-semibold">Self-Healing</h3>
                    <p className="text-sm text-muted-foreground">Auto-repair selectors when websites change</p>
                  </div>
                </div>
              </div>
            </div>
            
            <Card className="border-border/50 bg-slate-950/50 backdrop-blur-sm">
              <div className="p-1">
                <CodeBlock
                  language="bash"
                  title="Quick Installation"
                  code={`pip install crawlingo

from crawlingo import Page

page = Page("https://example.com")
print(page.title())
print(page.text())`}
                />
              </div>
            </Card>
          </div>
        </div>
      </section>

      {/* Architecture Preview */}
      <section className="py-20 relative bg-gradient-to-b from-background via-background to-muted/20">
        <div className="mx-auto max-w-7xl px-6 lg:px-8">
          <div className="mx-auto max-w-2xl text-center">
            <h2 className="text-3xl font-bold tracking-tight sm:text-4xl">
              Architecture Overview
            </h2>
            <p className="mt-4 text-lg text-muted-foreground">
              One unified engine, zero compromises.
            </p>
          </div>
          
          <div className="mt-16 flex flex-col items-center justify-center gap-8 lg:flex-row">
            <div className="flex-1 text-center">
              <div className="flex h-16 w-16 items-center justify-center rounded-2xl bg-gradient-to-br from-indigo-500 to-purple-600 mx-auto">
                <Terminal className="h-8 w-8 text-white" />
              </div>
              <h3 className="mt-4 font-semibold">Rust Core</h3>
              <p className="text-sm text-muted-foreground">Zero-copy, high-performance engine</p>
            </div>
            
            <div className="flex items-center">
              <div className="h-px w-16 bg-gradient-to-r from-indigo-500 to-purple-500" />
              <GitBranch className="h-6 w-6 text-indigo-400" />
              <div className="h-px w-16 bg-gradient-to-r from-purple-500 to-blue-500" />
            </div>
            
            <div className="flex-1 text-center">
              <div className="flex h-16 w-16 items-center justify-center rounded-2xl bg-gradient-to-br from-purple-500 to-blue-600 mx-auto">
                <Globe className="h-8 w-8 text-white" />
              </div>
              <h3 className="mt-4 font-semibold">SDKs</h3>
              <p className="text-sm text-muted-foreground">Python, Node.js, Rust</p>
            </div>
            
            <div className="flex items-center">
              <div className="h-px w-16 bg-gradient-to-r from-blue-500 to-teal-500" />
              <Webhook className="h-6 w-6 text-blue-400" />
              <div className="h-px w-16 bg-gradient-to-r from-teal-500 to-green-500" />
            </div>
            
            <div className="flex-1 text-center">
              <div className="flex h-16 w-16 items-center justify-center rounded-2xl bg-gradient-to-br from-teal-500 to-green-600 mx-auto">
                <Search className="h-8 w-8 text-white" />
              </div>
              <h3 className="mt-4 font-semibold">MCP Server</h3>
              <p className="text-sm text-muted-foreground">AI Integration Ready</p>
            </div>
          </div>
        </div>
      </section>

      {/* Call to Action */}
      <section className="py-20 relative overflow-hidden">
        <div className="absolute inset-0 bg-gradient-to-br from-indigo-900/50 via-purple-900/50 to-blue-900/50" />
        <div className="relative mx-auto max-w-4xl px-6 lg:px-8">
          <Card className="border-indigo-500/30 bg-gradient-to-br from-indigo-950/50 to-purple-950/50 backdrop-blur-sm">
            <CardContent className="p-12 text-center">
              <h2 className="text-3xl font-bold tracking-tight sm:text-4xl">
                Ready to Build Something Amazing?
              </h2>
              <p className="mt-4 text-lg text-muted-foreground">
                Join thousands of developers who rely on Crawlingo for their web scraping needs.
              </p>
              
              <div className="mt-10 flex flex-col items-center justify-center gap-4 sm:flex-row">
                <Link href="/quick-start">
                  <Button size="lg" className="w-full sm:w-auto bg-indigo-600 hover:bg-indigo-700">
                    Get Started Now
                  </Button>
                </Link>
                <Link href="https://github.com/Vamshavardhan50/crawlingo" target="_blank">
                  <Button size="lg" variant="outline" className="w-full sm:w-auto">
                    View on GitHub
                  </Button>
                </Link>
              </div>
            </CardContent>
          </Card>
        </div>
      </section>
    </div>
  );
}
