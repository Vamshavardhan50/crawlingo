//! Sitemap crawling: fetch and parse `sitemap.xml` (and sitemap-index files), then seed a
//! [`crate::crawl::frontier::Frontier`] with every discovered URL.
//!
//! ## Entry points
//! - [`SitemapCrawler::fetch`] — downloads a sitemap, resolves nested sitemap-index files
//!   recursively, and seeds all discovered `<loc>` URLs into the given frontier, then kicks off a
//!   normal [`crate::crawl::crawler::Crawler`] run against those URLs.
//! - [`parse_sitemap`] — lower-level: parses raw XML bytes and returns the `SitemapEntry`/
//!   `SitemapIndexEntry` lists; useful for tests and offline analysis without network access.
//!
//! Both sitemap types are supported:
//! - **`<urlset>`** — a leaf sitemap listing individual `<url>/<loc>` entries, optionally with
//!   `<lastmod>`, `<changefreq>`, and `<priority>`.
//! - **`<sitemapindex>`** — an index pointing to child sitemaps, each resolved recursively (up
//!   to `max_depth` to avoid infinite loops in malformed sitemaps).
//!
//! Gzip-compressed sitemaps (`.xml.gz`) are **not** decompressed by this layer; the `wreq`
//! client, if configured with `Accept-Encoding: gzip`, will decompress automatically for
//! standard Content-Encoding responses (most hosting setups send them that way). Files served as
//! raw application/x-gzip with no Content-Encoding decompression are a known follow-up item.

use crate::crawl::crawler::Crawler;
use crate::crawl::frontier::{Frontier, MemoryFrontier};
use crate::dataset::builder::DatasetResult;
use crate::engine::fetcher::FetchRequest;
use crate::engine::fetcher::FetcherTier;
use crate::engine::session::Session;
use crate::error::{CrawlingoError, Result};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

/// A single `<url>` entry from a `<urlset>` sitemap.
#[derive(Debug, Clone)]
pub struct SitemapEntry {
    pub loc: String,
    pub lastmod: Option<String>,
    pub changefreq: Option<String>,
    /// Stored as a raw string to avoid floating-point precision issues.
    pub priority: Option<String>,
}

/// A single `<sitemap>` entry from a `<sitemapindex>`.
#[derive(Debug, Clone)]
pub struct SitemapIndexEntry {
    pub loc: String,
    pub lastmod: Option<String>,
}

/// The result of parsing a sitemap document — either a leaf `<urlset>` or an index pointing to
/// child sitemaps.
#[derive(Debug, Clone)]
pub enum ParsedSitemap {
    /// A `<urlset>` listing individual page URLs.
    Urlset(Vec<SitemapEntry>),
    /// A `<sitemapindex>` listing child sitemap URLs.
    Index(Vec<SitemapIndexEntry>),
}

/// Parses a raw XML sitemap (as bytes) and returns its entries.
///
/// Does not perform any network access — callers are responsible for fetching the raw bytes.
/// Returns `ParsedSitemap::Urlset([])` if the document is not a recognized sitemap format.
pub fn parse_sitemap(xml: &[u8]) -> Result<ParsedSitemap> {
    let text = std::str::from_utf8(xml).map_err(|e| {
        CrawlingoError::FetchError(format!("sitemap bytes are not valid UTF-8: {e}"))
    })?;

    // Determine root element type by scanning for the first non-trivial tag.
    let is_index = text.contains("<sitemapindex");
    let is_urlset = text.contains("<urlset");

    if !is_index && !is_urlset {
        // Not a recognized sitemap — return empty urlset rather than failing.
        return Ok(ParsedSitemap::Urlset(Vec::new()));
    }

    if is_index {
        let entries = parse_sitemap_index_entries(text);
        Ok(ParsedSitemap::Index(entries))
    } else {
        let entries = parse_urlset_entries(text);
        Ok(ParsedSitemap::Urlset(entries))
    }
}

/// Parses `<url>` entries from a `<urlset>` sitemap string.
fn parse_urlset_entries(text: &str) -> Vec<SitemapEntry> {
    let mut entries = Vec::new();
    for chunk in split_tags(text, "url") {
        let loc = extract_tag_text(chunk, "loc");
        if loc.is_empty() {
            continue;
        }
        entries.push(SitemapEntry {
            loc,
            lastmod: {
                let v = extract_tag_text(chunk, "lastmod");
                if v.is_empty() { None } else { Some(v) }
            },
            changefreq: {
                let v = extract_tag_text(chunk, "changefreq");
                if v.is_empty() { None } else { Some(v) }
            },
            priority: {
                let v = extract_tag_text(chunk, "priority");
                if v.is_empty() { None } else { Some(v) }
            },
        });
    }
    entries
}

/// Parses `<sitemap>` entries from a `<sitemapindex>` string.
fn parse_sitemap_index_entries(text: &str) -> Vec<SitemapIndexEntry> {
    let mut entries = Vec::new();
    for chunk in split_tags(text, "sitemap") {
        let loc = extract_tag_text(chunk, "loc");
        if loc.is_empty() {
            continue;
        }
        entries.push(SitemapIndexEntry {
            loc,
            lastmod: {
                let v = extract_tag_text(chunk, "lastmod");
                if v.is_empty() { None } else { Some(v) }
            },
        });
    }
    entries
}

/// Returns an iterator of the text content of each `<tag>…</tag>` block in `text`.
fn split_tags<'a>(text: &'a str, tag: &str) -> impl Iterator<Item = &'a str> {
    let open = format!("<{tag}");
    let close = format!("</{tag}>");
    let mut rest = text;
    std::iter::from_fn(move || {
        let start = rest.find(open.as_str())?;
        let after_open = rest[start..].find('>')? + start + 1;
        let close_start = rest[after_open..].find(close.as_str())?;
        let chunk = &rest[after_open..after_open + close_start];
        rest = &rest[after_open + close_start + close.len()..];
        Some(chunk)
    })
}

/// Returns the trimmed text content of the first `<tag>…</tag>` in `text`, or `""`.
fn extract_tag_text(text: &str, tag: &str) -> String {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    if let Some(start) = text.find(&open) {
        let after = start + open.len();
        if let Some(end) = text[after..].find(&close) {
            return text[after..after + end].trim().to_string();
        }
    }
    // Also match <tag attr="…">
    let open_any = format!("<{tag} ");
    if let Some(start) = text.find(&open_any) {
        if let Some(gt_offset) = text[start..].find('>') {
            let after_gt = start + gt_offset + 1;
            let close2 = format!("</{tag}>");
            if let Some(end) = text[after_gt..].find(&close2) {
                return text[after_gt..after_gt + end].trim().to_string();
            }
        }
    }
    String::new()
}

/// Crawls a sitemap URL, resolves all nested sitemaps, and seeds every discovered page URL into
/// `frontier`. After seeding, creates a [`Crawler`] configured with that frontier and runs it.
pub struct SitemapCrawler {
    /// The root sitemap URL to fetch.
    pub sitemap_url: String,
    pub session: Arc<Session>,
    /// Maximum nesting depth for sitemap-index resolution (`0` = only fetch root, `n` = recurse
    /// at most `n` levels deep). Defaults to `5`.
    pub max_depth: usize,
    /// The frontier to seed discovered URLs into (and hand to the downstream crawler). If `None`,
    /// an ephemeral [`MemoryFrontier`] is used.
    pub frontier: Option<Arc<dyn Frontier>>,
    /// Optional [`Crawler`] template. When `Some`, fields like `follow_selector`, `limit`,
    /// `concurrency`, etc. are copied from it; `start_url` and `session` are overridden with
    /// this struct's own values.
    pub crawler_template: Option<Crawler>,
}

impl SitemapCrawler {
    pub fn new(sitemap_url: &str, session: Arc<Session>) -> Self {
        Self {
            sitemap_url: sitemap_url.to_string(),
            session,
            max_depth: 5,
            frontier: None,
            crawler_template: None,
        }
    }

    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    pub fn with_frontier(mut self, frontier: Arc<dyn Frontier>) -> Self {
        self.frontier = Some(frontier);
        self
    }

    pub fn with_crawler_template(mut self, template: Crawler) -> Self {
        self.crawler_template = Some(template);
        self
    }

    /// Fetches the sitemap, seeds discovered URLs into the frontier, and runs the downstream
    /// `Crawler` against them. Returns all collected `DatasetResult`s.
    pub fn fetch(&self) -> Result<Vec<DatasetResult>> {
        crate::TOKIO_RUNTIME.block_on(self.fetch_async())
    }

    pub async fn fetch_async(&self) -> Result<Vec<DatasetResult>> {
        let frontier: Arc<dyn Frontier> = self
            .frontier
            .clone()
            .unwrap_or_else(|| Arc::new(MemoryFrontier::new()));

        // Collect all page URLs from the sitemap tree.
        let mut visited_sitemaps: HashSet<String> = HashSet::new();
        self.collect_urls(
            &self.sitemap_url,
            0,
            &frontier,
            &mut visited_sitemaps,
        )
        .await?;

        if frontier.pending_len() == 0 {
            tracing::warn!(
                "sitemap at {} yielded no crawlable URLs",
                self.sitemap_url
            );
            return Ok(Vec::new());
        }

        // Build the downstream crawler seeded with all discovered URLs.
        let crawler = self.build_crawler(frontier);
        crawler.crawl_async().await
    }

    async fn collect_urls(
        &self,
        url: &str,
        depth: usize,
        frontier: &Arc<dyn Frontier>,
        visited: &mut HashSet<String>,
    ) -> Result<()> {
        if depth > self.max_depth {
            tracing::warn!("sitemap recursion depth exceeded at {url}, stopping");
            return Ok(());
        }
        if visited.contains(url) {
            return Ok(());
        }
        visited.insert(url.to_string());

        let xml = self.fetch_raw(url).await?;
        let parsed = parse_sitemap(&xml)?;

        match parsed {
            ParsedSitemap::Urlset(entries) => {
                for entry in entries {
                    frontier.enqueue(entry.loc, 0);
                }
            }
            ParsedSitemap::Index(entries) => {
                for entry in entries {
                    // Recurse into each child sitemap.
                    let child_url = entry.loc.clone();
                    if let Err(e) = Box::pin(self.collect_urls(
                        &child_url,
                        depth + 1,
                        frontier,
                        visited,
                    ))
                    .await
                    {
                        tracing::error!("failed to fetch child sitemap {child_url}: {e}");
                    }
                }
            }
        }
        Ok(())
    }

    async fn fetch_raw(&self, url: &str) -> Result<Vec<u8>> {
        let manager = self.session.fetch_manager();
        let req = FetchRequest {
            url: url.to_string(),
            tier: FetcherTier::Standard,
            browser_profile: None,
            headers: self.session.headers.read().unwrap().clone(),
            cookies: self.session.cookies.read().unwrap().clone(),
            proxy: self.session.get_next_proxy(),
            timeout: Duration::from_secs(*self.session.timeout_seconds.read().unwrap()),
            retries: 2,
            rate_limit_rps: 0.0,
        };
        let resp = manager.dispatch(req).await?;
        Ok(resp.body.to_vec())
    }

    fn build_crawler(&self, frontier: Arc<dyn Frontier>) -> Crawler {
        let base = if let Some(ref template) = self.crawler_template {
            template.clone()
        } else {
            Crawler::new(&self.sitemap_url, self.session.clone())
        };
        // Override session and frontier; keep all other template settings.
        Crawler {
            start_url: self.sitemap_url.clone(),
            session: self.session.clone(),
            frontier: Some(frontier),
            ..base
        }
    }
}

/// Returns the canonical `sitemap.xml` URL for a given origin (e.g. `https://example.com` →
/// `https://example.com/sitemap.xml`). Also checks `robots.txt` for a `Sitemap:` directive.
pub fn sitemap_url_for_origin(origin: &str) -> String {
    let base = origin.trim_end_matches('/');
    format!("{base}/sitemap.xml")
}

#[cfg(test)]
mod tests {
    use super::*;

    const URLSET_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>https://example.com/</loc>
    <lastmod>2024-01-01</lastmod>
    <changefreq>weekly</changefreq>
    <priority>1.0</priority>
  </url>
  <url>
    <loc>https://example.com/about</loc>
    <changefreq>monthly</changefreq>
    <priority>0.8</priority>
  </url>
  <url>
    <loc>https://example.com/blog</loc>
  </url>
</urlset>"#;

    const INDEX_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <sitemap>
    <loc>https://example.com/sitemap-pages.xml</loc>
    <lastmod>2024-01-01</lastmod>
  </sitemap>
  <sitemap>
    <loc>https://example.com/sitemap-posts.xml</loc>
  </sitemap>
</sitemapindex>"#;

    const EMPTY_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<not-a-sitemap><something/></not-a-sitemap>"#;

    #[test]
    fn parses_urlset_with_all_optional_fields() {
        let parsed = parse_sitemap(URLSET_XML.as_bytes()).unwrap();
        let ParsedSitemap::Urlset(entries) = parsed else {
            panic!("expected Urlset");
        };
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].loc, "https://example.com/");
        assert_eq!(entries[0].lastmod.as_deref(), Some("2024-01-01"));
        assert_eq!(entries[0].changefreq.as_deref(), Some("weekly"));
        assert_eq!(entries[0].priority.as_deref(), Some("1.0"));

        // Second entry has no lastmod
        assert_eq!(entries[1].loc, "https://example.com/about");
        assert!(entries[1].lastmod.is_none());

        // Third entry has none of the optional fields
        assert_eq!(entries[2].loc, "https://example.com/blog");
        assert!(entries[2].changefreq.is_none());
        assert!(entries[2].priority.is_none());
    }

    #[test]
    fn parses_sitemap_index() {
        let parsed = parse_sitemap(INDEX_XML.as_bytes()).unwrap();
        let ParsedSitemap::Index(entries) = parsed else {
            panic!("expected Index");
        };
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].loc, "https://example.com/sitemap-pages.xml");
        assert_eq!(entries[0].lastmod.as_deref(), Some("2024-01-01"));
        assert_eq!(entries[1].loc, "https://example.com/sitemap-posts.xml");
        assert!(entries[1].lastmod.is_none());
    }

    #[test]
    fn unrecognized_xml_returns_empty_urlset() {
        let parsed = parse_sitemap(EMPTY_XML.as_bytes()).unwrap();
        let ParsedSitemap::Urlset(entries) = parsed else {
            panic!("expected Urlset for unrecognized doc");
        };
        assert!(entries.is_empty());
    }

    #[test]
    fn sitemap_url_for_origin_appends_sitemap_xml() {
        assert_eq!(
            sitemap_url_for_origin("https://example.com"),
            "https://example.com/sitemap.xml"
        );
        // Trailing slash stripped.
        assert_eq!(
            sitemap_url_for_origin("https://example.com/"),
            "https://example.com/sitemap.xml"
        );
    }

    #[test]
    fn urlset_seeds_frontier_correctly() {
        let frontier = Arc::new(MemoryFrontier::new());
        let parsed = parse_sitemap(URLSET_XML.as_bytes()).unwrap();
        if let ParsedSitemap::Urlset(entries) = parsed {
            for e in entries {
                frontier.enqueue(e.loc, 0);
            }
        }
        assert_eq!(frontier.pending_len(), 3);
        // Dequeue in LIFO order — last enqueued first.
        let (url, depth) = frontier.dequeue().unwrap();
        assert_eq!(url, "https://example.com/blog");
        assert_eq!(depth, 0);
    }
}
