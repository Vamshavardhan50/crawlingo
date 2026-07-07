import React from 'react';
import Link from 'next/link';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';
import { CodeBlock, TerminalExample } from '@/components/ui-card';
import { StatusBadge } from '@/components/feature-card';

export default function QuickStartPage() {
  return (
    <div>
      {/* Page Header */}
      <div className="mb-12">
        <h1 className="text-4xl font-bold tracking-tight sm:text-5xl">Quick Start</h1>
        <p className="mt-4 text-lg text-muted-foreground">
          Get up and running with Crawlingo in under 2 minutes.
        </p>
      </div>

      {/* Installation */}
      <section className="mb-12">
        <h2 className="text-2xl font-bold tracking-tight">Installation</h2>
        <p className="mt-2 text-muted-foreground">
          Install the Python package to get started.
        </p>
        
        <div className="mt-6 space-y-4">
          <TerminalExample
            title="Install Crawlingo"
            commands={["pip install crawlingo"]}
          />
          
          <TerminalExample
            title="Verify Installation"
            commands={['python -c "import crawlingo; print(crawlingo.__version__)"']}
          />
        </div>
      </section>

      {/* Basic Usage */}
      <section className="mb-12">
        <h2 className="text-2xl font-bold tracking-tight">Basic Usage</h2>
        <p className="mt-2 text-muted-foreground">
          Extract data from any website with a simple API.
        </p>
        
        <div className="mt-6 space-y-6">
          <Card className="border-border/50 bg-slate-950/30">
            <CardHeader>
              <CardTitle>Simple Fetch</CardTitle>
              <CardDescription>Fetch and extract page contents in a single call</CardDescription>
            </CardHeader>
            <CardContent>
              <CodeBlock
                language="python"
                title="Basic Page Extraction"
                code={`from crawlingo import Page

# Fetch and extract page contents in a single call
page = Page("https://example.com")

print(page.title())
print(page.text())`}
              />
            </CardContent>
          </Card>

          <Card className="border-border/50 bg-slate-950/30">
            <CardHeader>
              <CardTitle>CSS Selectors</CardTitle>
              <CardDescription>Use CSS selectors to extract specific elements</CardDescription>
            </CardHeader>
            <CardContent>
              <CodeBlock
                language="python"
                title="CSS Selector Examples"
                code={`from crawlingo import Page

page = Page("https://example.com")

# Get the title
h1 = page.css("h1").text()

# Iterate through paragraphs
for p in page.xpath("//p"):
    print(p.text())

# Get all links
for link in page.css("a"):
    print(link.attr("href"))`}
              />
            </CardContent>
          </Card>
        </div>
      </section>

      {/* Session Configuration */}
      <section className="mb-12">
        <h2 className="text-2xl font-bold tracking-tight">Session Configuration</h2>
        <p className="mt-2 text-muted-foreground">
          Configure session behavior for your specific needs.
        </p>
        
        <div className="mt-6 space-y-4">
          <TerminalExample
            title="Configure Session"
            commands={[`from crawlingo import Session

with Session() as session:
    session.auto_match(True)
    session.timeout(30)
    session.fetcher_tier("stealthy")
    session.browser_profile("chrome")
    session.rate_limit(3.0)

    page = session.page("https://example.com")
    print(page.css("h1").text())`]}
          />
        </div>
      </section>

      {/* Dataset Building */}
      <section className="mb-12">
        <h2 className="text-2xl font-bold tracking-tight">Dataset Building</h2>
        <p className="mt-2 text-muted-foreground">
          Build structured datasets with ease.
        </p>
        
        <div className="mt-6 space-y-4">
          <Card className="border-border/50 bg-slate-950/30">
            <CardHeader>
              <CardTitle>Build a Dataset</CardTitle>
              <CardDescription>Extract structured data and export to multiple formats</CardDescription>
            </CardHeader>
            <CardContent>
              <CodeBlock
                language="python"
                title="Dataset Builder Example"
                code={`from crawlingo import Dataset

ds = (
    Dataset("https://example.com")
    .auto_match(True)
    .field("title", "h1")
    .field("price", ".price", selector_type="css", transform=lambda v: v.replace("$", ""))
    .build()
)

print(ds.to_dict())
ds.to_json("data.json")
ds.to_csv("data.csv")
ds.to_parquet("data.parquet")`}
              />
            </CardContent>
          </Card>
        </div>
      </section>

      {/* Crawler Example */}
      <section className="mb-12">
        <h2 className="text-2xl font-bold tracking-tight">Batch Processing</h2>
        <p className="mt-2 text-muted-foreground">
          Process multiple pages efficiently with the Crawler.
        </p>
        
        <div className="mt-6 space-y-4">
          <Card className="border-border/50 bg-slate-950/30">
            <CardHeader>
              <CardTitle>Crawler Usage</CardTitle>
              <CardDescription>Process multiple pages with a single configuration</CardDescription>
            </CardHeader>
            <CardContent>
              <CodeBlock
                language="python"
                title="Crawler Example"
                code={`from crawlingo import Crawl

results = (
    Crawl("https://example.com")
    .follow("a.next-page")
    .limit(100)
    .depth(5)
    .concurrency(4)
    .delay(0.5)
    .field("title", "h1")
    .webhook("https://api.example.com/webhooks/crawl")
    .build()
)

results.to_json("output.json")`}
              />
            </CardContent>
          </Card>
        </div>
      </section>

      {/* Next Steps */}
      <section className="mt-16">
        <Card className="border-indigo-500/30 bg-gradient-to-r from-indigo-950/30 to-purple-950/30">
          <CardHeader>
            <CardTitle>What's Next?</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="grid gap-4 sm:grid-cols-2">
              <Link href="/features/auto-match">
                <div className="group rounded-lg border border-border/50 bg-card/50 p-4 transition-colors hover:bg-muted/50">
                  <h3 className="font-semibold">Learn About Self-Healing</h3>
                  <p className="text-sm text-muted-foreground">Understand how Crawlingo auto-repairs selectors</p>
                </div>
              </Link>
              
              <Link href="/sdk/python">
                <div className="group rounded-lg border border-border/50 bg-card/50 p-4 transition-colors hover:bg-muted/50">
                  <h3 className="font-semibold">SDK Documentation</h3>
                  <p className="text-sm text-muted-foreground">Explore Python, Node.js, and Rust SDKs</p>
                </div>
              </Link>
              
              <Link href="/tools/fetch-page">
                <div className="group rounded-lg border border-border/50 bg-card/50 p-4 transition-colors hover:bg-muted/50">
                  <h3 className="font-semibold">API Reference</h3>
                  <p className="text-sm text-muted-foreground">Complete API documentation for all tools</p>
                </div>
              </Link>
              
              <Link href="/changelog">
                <div className="group rounded-lg border border-border/50 bg-card/50 p-4 transition-colors hover:bg-muted/50">
                  <h3 className="font-semibold">Changelog</h3>
                  <p className="text-sm text-muted-foreground">View recent updates and improvements</p>
                </div>
              </Link>
            </div>
          </CardContent>
        </Card>
      </section>
    </div>
  );
}
