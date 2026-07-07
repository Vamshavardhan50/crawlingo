import React from 'react';
import Link from 'next/link';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { FeatureCard, CodeBlock } from '@/components/feature-card';
import { Shield, Search, Cpu, GitBranch, Activity, Database, FileText, Layers, Webhook, Globe } from 'lucide-react';
  return (
    <div>
      {/* Page Header */}
      <div className="mb-12">
        <h1 className="text-4xl font-bold tracking-tight sm:text-5xl">Features</h1>
        <p className="mt-4 text-lg text-muted-foreground">
          Discover the powerful features that make Crawlingo the ultimate web scraping solution.
        </p>
      </div>

      {/* Self-Healing DOM Fingerprinting */}
      <section className="mb-16">
        <div className="grid gap-8 lg:grid-cols-2 lg:gap-12 items-center">
          <div>
            <h2 className="text-3xl font-bold tracking-tight">Self-Healing DOM Fingerprinting</h2>
            <p className="mt-4 text-muted-foreground">
              When websites change their class names, IDs, or HTML structures (selector drift), Crawlingo solves this by caching element layout fingerprints and using similarity matching heuristics to self-heal and find drifted elements on the fly.
            </p>
            
            <div className="mt-8 space-y-6">
              <div className="flex gap-4">
                <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-indigo-500/10 text-indigo-400">
                  <Search className="h-5 w-5" />
                </div>
                <div>
                  <h3 className="font-semibold">Drift Detection</h3>
                  <p className="text-sm text-muted-foreground">Identifies when target elements undergo styling/structure changes</p>
                </div>
              </div>
              
              <div className="flex gap-4">
                <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-purple-500/10 text-purple-400">
                  <Layers className="h-5 w-5" />
                </div>
                <div>
                  <h3 className="font-semibold">Dynamic Fingerprinting</h3>
                  <p className="text-sm text-muted-foreground">Captures candidate nodes within parent coordinates when mismatches occur</p>
                </div>
              </div>
             n              <div className="flex gap-4">
                <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-blue-500/10 text-blue-400">
                  <Cpu className="h-5 w-5" />
                </div>
                <div>
                  <h3 className="font-semibold">Similarity Scoring</h3>
                  <p className="text-sm text-muted-foreground">Uses Jaro-Winkler and Jaccard scoring to rank candidates by similarity</p>
                </div>
              </div>
            </div>
          </div>
          
          <Card className="border-border/50 bg-slate-950/30">
            <CardHeader>
              <CardTitle>How Self-Healing Works</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="space-y-4 text-sm">
                <div className="flex items-center gap-3">
                  <div className="flex h-6 w-6 items-center justify-center rounded-full bg-indigo-500/20 text-indigo-400">1</div>
                  <span>When <code className="text-indigo-400">button#submit.btn-primary</code> cannot be matched</span>
                </div>
                <div className="flex items-center gap-3">
                  <div className="flex h-6 w-6 items-center justify-center rounded-full bg-purple-500/20 text-purple-400">2</div>
                  <span>Rust engine intercepts the mismatch, loads active DOM, isolates candidates within parent coordinates</span>
                </div>
                <div className="flex items-center gap-3">
                  <div className="flex h-6 w-6 items-center justify-center rounded-full bg-blue-500/20 text-blue-400">3</div>
                  <span>Compares tag names, attributes, text contents, and structural fingerprints</span>
                </div>
                <div className="flex items-center gap-3">
                  <div className="flex h-6 w-6 items-center justify-center rounded-full bg-green-500/20 text-green-400">4</div>
                  <span>Automatically binds element with highest similarity (>94% confidence), updating cache</span>
                </div>
              </div>
            </CardContent>
          </Card>
        </div>
      </section>

      {/* Stealth Browser Impersonation */}
      <section className="mb-16">
        <div className="grid gap-8 lg:grid-cols-2 lg:gap-12 items-center">
          <Card className="order-2 lg:order-1 border-border/50 bg-slate-950/30">
            <CardHeader>
              <CardTitle>Stealth Browser Impersonation</CardTitle>
              <CardDescription>
                Compiles a raw HTTP/2 client inside Rust to rotate JA3/TLS handshake fingerprints, user-agent profiles, and request timing gaps.
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                <div className="flex items-start gap-3">
                  <div className="mt-1 flex h-5 w-5 items-center justify-center rounded-full bg-indigo-500/20 text-indigo-400">✓</div>
                  <div>
                    <h4 className="font-medium">Bypasses Cloudflare Turnstile</h4>
                    <p className="text-sm text-muted-foreground">Advanced web verification with minimal footprint</p>
                  </div>
                </div>
                <div className="flex items-start gap-3">
                  <div className="mt-1 flex h-5 w-5 items-center justify-center rounded-full bg-purple-500/20 text-purple-400">✓</div>
                  <div>
                    <h4 className="font-medium">Zero Headless Browser Overhead</h4>
                    <p className="text-sm text-muted-foreground">Raw HTTP/2 client instead of Puppeteer/Playwright</p>
                  </div>
                </div>
                <div className="flex items-start gap-3">
                  <div className="mt-1 flex h-5 w-5 items-center justify-center rounded-full bg-blue-500/20 text-blue-400">✓</div>
                  <div>
                    <h4 className="font-medium">Multi-Profile Rotation</h4>
                    <p className="text-sm text-muted-foreground">Chrome, Firefox, Safari browser profiles with TLS fingerprints</p>
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>
          
          <div className="order-1 lg:order-2">
            <h2 className="text-3xl font-bold tracking-tight">Stealth Browser Impersonation</h2>
            <p className="mt-4 text-muted-foreground">
              Compile a raw HTTP/2 client inside Rust to rotate JA3/TLS handshake fingerprints, user-agent profiles, and request timing gaps.
            </p>
            
            <div className="mt-8 space-y-4">
              <div className="flex gap-4">
                <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-indigo-500/10 text-indigo-400">
                  <Globe className="h-5 w-5" />
                </div>
                <div>
                  <h3 className="font-semibold">Multi-Profile Support</h3>
                  <p className="text-sm text-muted-foreground">Chrome, Firefox, Safari browser profiles</p>
                </div>
              </div>
              
              <div className="flex gap-4">
                <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-purple-500/10 text-purple-400">
                  <GitBranch className="h-5 w-5" />
                </div>
                <div>
                  <h3 className="font-semibold">TLS Fingerprint Rotation</h3>
                  <p className="text-sm text-muted-foreground">JA3/TLS handshake fingerprint rotation</p>
                </div>
              </div>
              
              <div className="flex gap-4">
                <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-blue-500/10 text-blue-400">
                  <FileText className="h-5 w-5" />
                </div>
                <div>
                  <h3 className="font-semibold">Timing Gaps</h3>
                  <p className="text-sm text-muted-foreground">Request timing randomizations to avoid detection</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* SIMD-Accelerated Text Anchors */}
      <section className="mb-16">
        <div className="grid gap-8 lg:grid-cols-2 lg:gap-12 items-center">
          <div>
            <h2 className="text-3xl font-bold tracking-tight">SIMD-Accelerated Text Anchors</h2>
            <p className="mt-4 text-muted-foreground">
              Search text occurrences and targets neighboring nodes using parallel CPU vectors, facilitating simple table extraction when class structures are non-existent.
            </p>
            
            <div className="mt-8 space-y-6">
              <div className="flex gap-4">
                <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-indigo-500/10 text-indigo-400">
                  <Cpu className="h-5 w-5" />
                </div>
                <div>
                  <h3 className="font-semibold">SIMD Processing</h3>
                  <p className="text-sm text-muted-foreground">Parallel CPU vectors for fast text searches</p>
                </div>
              </div>
              
              <div className="flex gap-4">
                <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-purple-500/10 text-purple-400">
                  <Search className="h-5 w-5" />
                </div>
                <div>
                  <h3 className="font-semibold">Multiple Anchor Types</h3>
                  <p className="text-sm text-muted-foreground">Text, after_text, before_text for robust element targeting</p>
                </div>
              </div>
              
              <div className="flex gap-4">
                <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-blue-500/10 text-blue-400">
                  <Database className="h-5 w-5" />
                </div>
                <div>
                  <h3 className="font-semibold">Simple Table Extraction</h3>
                  <p className="text-sm text-muted-foreground">No class structures required for data extraction</p>
                </div>
              </div>
            </div>
          </div>
          
          <Card className="border-border/50 bg-slate-950/30">
            <CardHeader>
              <CardTitle>Text Anchor Types</CardTitle>
            </CardHeader>
            <CardContent>
              <CodeBlock
                language="python"
                title="Text Anchor Examples"
                code={`from crawlingo import Page

page = Page("https://example.com")

# Find elements by literal text content
element = page.find_text("Important Price")

# Elements that come after specific text
elements = page.after_text("Price: $")

# Elements that come before specific text  
elements = page.before_text("USD")`}
              />
            </CardContent>
          </Card>
        </div>
      </section>

      {/* Change Monitoring */}
      <section className="mb-16">
        <div className="grid gap-8 lg:grid-cols-2 lg:gap-12 items-center">
          <Card className="order-2 lg:order-1 border-border/50 bg-slate-950/30">
            <CardHeader>
              <CardTitle>Reactive Watch Monitors</CardTitle>
              <CardDescription>
                Tracks target DOM nodes on interval loops, instantly publishing callbacks or webhooks when price quotes, stock changes, or layout modifications are discovered.
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                <div className="flex items-start gap-3">
                  <div className="mt-1 flex h-5 w-5 items-center justify-center rounded-full bg-green-500/20 text-green-400">✓</div>
                  <div>
                    <h4 className="font-medium">Price Change Detection</h4>
                    <p className="text-sm text-muted-foreground">Detects when prices increase/decrease</p>
                  </div>
                </div>
                <div className="flex items-start gap-3">
                  <div className="mt-1 flex h-5 w-5 items-center justify-center rounded-full bg-red-500/20 text-red-400">✓</div>
                  <div>
                    <h4 className="font-medium">Stock Status Monitoring</h4>
                    <p className="text-sm text-muted-foreground">Tracks in-stock vs out-of-stock changes</p>
                  </div>
                </div>
                <div className="flex items-start gap-3">
                  <div className="mt-1 flex h-5 w-5 items-center justify-center rounded-full bg-blue-500/20 text-blue-400">✓</div>
                  <div>
                    <h4 className="font-medium">Element Change Detection</h4>
                    <p className="text-sm text-muted-foreground">Monitors when elements are added/removed</p>
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>
          
          <div className="order-1 lg:order-2">
            <h2 className="text-3xl font-bold tracking-tight">Reactive Watch Monitors</h2>
            <p className="mt-4 text-muted-foreground">
              Tracks target DOM nodes on interval loops, instantly publishing callbacks or webhooks when price quotes, stock changes, or layout modifications are discovered.
            </p>
            
            <div className="mt-8 space-y-6">
              <div className="flex gap-4">
                <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-green-500/10 text-green-400">
                  <Webhook className="h-5 w-5" />
                </div>
                <div>
                  <h3 className="font-semibold">Webhook Integration</h3>
                  <p className="text-sm text-muted-foreground">Publish changes to external services instantly</p>
                </div>
              </div>
             n              <div className="flex gap-4">
                <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-red-500/10 text-red-400">
                  <Activity className="h-5 w-5" />
                </div>
                <div>
                  <h3 className="font-semibold">Event Callbacks</h3>
                  <p className="text-sm text-muted-foreground">Multiple callback types (on_change, on_price_change, on_stock_change)</p>
                </div>
              </div>
              
              <div className="flex gap-4">
                <div className="flex h-10 w-10 items-center justify-center rounded-lg bg-blue-500/10 text-blue-400">
                  <Clock className="h-5 w-5" />
                </div>
                <div>
                  <h3 className="font-semibold">Interval Polling</h3>
                  <p className="text-sm text-muted-foreground">Configurable intervals for monitoring</p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>
    </div>
  );
}
