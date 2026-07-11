//! Automatic pagination: detect "next page" links and enqueue them into a
//! [`crate::crawl::frontier::Frontier`], enabling a [`crate::crawl::crawler::Crawler`] to follow
//! paginated content (numbered pages, cursor-based "load more" links, etc.) without the caller
//! having to know the pagination scheme in advance.
//!
//! ## Usage
//!
//! ```rust,ignore
//! let mut crawler = Crawler::new("https://example.com/blog", session);
//! // Tell the crawler which link selector leads to the next page.
//! crawler = crawler
//!     .follow("a.post-title")      // extract fields from each page
//!     .with_pagination(PaginationConfig::next_link("a[rel=next]"));
//! ```
//!
//! ## Supported schemes
//! - [`PaginationScheme::NextLink`] — follows `href` of the first element matching a CSS selector
//!   (e.g. `rel="next"`, `class="next-page"`, `aria-label="Next"`).
//! - [`PaginationScheme::PageNumber`] — constructs numbered URLs from a template string with a
//!   `{page}` placeholder (e.g. `https://example.com/blog?page={page}`), starting from a
//!   configurable start page and stopping at `max_pages`.
//! - [`PaginationScheme::UrlPattern`] — uses a regex to detect the current page number in the
//!   URL and increment it, e.g. `/page/(\d+)/` → `/page/2/`.
//!
//! The paginator is applied inside the crawler loop: after each page is successfully fetched and
//! field-extracted, the paginator decides whether to enqueue a "next" URL into the frontier.

use crate::crawl::frontier::Frontier;
use crate::error::{CrawlingoError, Result};
use crate::parser::document::DomTree;
use crate::selector::css;
use regex::Regex;
use std::sync::Arc;

/// How to determine the next page's URL from the current one.
#[derive(Debug, Clone)]
pub enum PaginationScheme {
    /// Follow the `href` of the first element matching `selector`. Commonly `a[rel="next"]`,
    /// `a.next-page`, etc.
    NextLink { selector: String },
    /// Construct numbered URLs from a template (must contain `{page}`). Starts at `start_page`
    /// and increments by 1 up to `max_pages` total pages.
    PageNumber {
        url_template: String,
        start_page: usize,
        max_pages: usize,
    },
    /// Finds a page number in the current URL via `page_regex` (first capture group), increments
    /// it, and replaces the match. Stops when the extracted number reaches `max_page`.
    UrlPattern {
        page_regex: String,
        max_page: usize,
    },
}

/// Configuration for automatic pagination within a crawl.
#[derive(Debug, Clone)]
pub struct PaginationConfig {
    pub scheme: PaginationScheme,
    /// If `true`, the paginator skips a "next" URL that the visited set already contains (always
    /// `true`; exposed as a config knob for testing).
    pub skip_visited: bool,
}

impl PaginationConfig {
    /// Convenience constructor: follow the first `href` of elements matching `selector`.
    pub fn next_link(selector: &str) -> Self {
        Self {
            scheme: PaginationScheme::NextLink {
                selector: selector.to_string(),
            },
            skip_visited: true,
        }
    }

    /// Convenience constructor: numbered pages via a URL template with `{page}`.
    pub fn page_number(url_template: &str, start_page: usize, max_pages: usize) -> Self {
        Self {
            scheme: PaginationScheme::PageNumber {
                url_template: url_template.to_string(),
                start_page,
                max_pages,
            },
            skip_visited: true,
        }
    }

    /// Convenience constructor: increment the page number captured by `page_regex`.
    pub fn url_pattern(page_regex: &str, max_page: usize) -> Self {
        Self {
            scheme: PaginationScheme::UrlPattern {
                page_regex: page_regex.to_string(),
                max_page,
            },
            skip_visited: true,
        }
    }
}

/// Determines the next page URL (if any), given the current page's URL and its parsed DOM.
pub struct Paginator {
    config: PaginationConfig,
    /// For `PageNumber`: tracks which page we're currently generating.
    page_counter: std::sync::atomic::AtomicUsize,
}

impl Paginator {
    pub fn new(config: PaginationConfig) -> Self {
        let start = if let PaginationScheme::PageNumber { start_page, .. } = &config.scheme {
            *start_page
        } else {
            1
        };
        Self {
            config,
            page_counter: std::sync::atomic::AtomicUsize::new(start),
        }
    }

    /// Computes the next page URL given the current `url` and its `dom`.
    ///
    /// Returns `Ok(None)` if pagination is exhausted or no next link was found.
    pub fn next_url(&self, url: &str, dom: &Arc<DomTree>) -> Result<Option<String>> {
        match &self.config.scheme {
            PaginationScheme::NextLink { selector } => {
                let matches = css::query(dom, selector);
                let next = matches.iter().find_map(|&idx| {
                    dom.nodes[idx].attrs.get("href").cloned()
                });
                if let Some(href) = next {
                    let resolved = resolve_url(url, &href);
                    Ok(resolved)
                } else {
                    Ok(None)
                }
            }

            PaginationScheme::PageNumber { url_template, start_page: _, max_pages } => {
                let current =
                    self.page_counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                // current is the *next* page number to generate (already incremented above).
                if current > *max_pages {
                    return Ok(None);
                }
                let next_url = url_template.replace("{page}", &current.to_string());
                Ok(Some(next_url))
            }

            PaginationScheme::UrlPattern { page_regex, max_page } => {
                let re = Regex::new(page_regex).map_err(|e| {
                    CrawlingoError::FetchError(format!("invalid pagination regex: {e}"))
                })?;
                if let Some(caps) = re.captures(url) {
                    let matched = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                    if let Ok(n) = matched.parse::<usize>() {
                        let next_n = n + 1;
                        if next_n > *max_page {
                            return Ok(None);
                        }
                        // Replace the matched capture group with n+1.
                        let full_match = caps.get(0).unwrap();
                        let new_segment = re.replace(url, full_match.as_str().replace(matched, &next_n.to_string()));
                        return Ok(Some(new_segment.into_owned()));
                    }
                }
                Ok(None)
            }
        }
    }

    /// Enqueues the next page URL (if any) into `frontier`, unless it's already visited and
    /// `skip_visited` is set.
    pub fn enqueue_next(
        &self,
        url: &str,
        dom: &Arc<DomTree>,
        frontier: &Arc<dyn Frontier>,
    ) -> Result<bool> {
        let next = self.next_url(url, dom)?;
        if let Some(next_url) = next {
            if self.config.skip_visited && frontier.is_visited(&next_url) {
                return Ok(false);
            }
            frontier.enqueue(next_url, 0);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

/// Resolves `href` (possibly relative) against `base_url`.
fn resolve_url(base: &str, href: &str) -> Option<String> {
    if href.starts_with("http://") || href.starts_with("https://") {
        return Some(href.to_string());
    }
    let base_url = url::Url::parse(base).ok()?;
    base_url.join(href).ok().map(|u| u.to_string())
}

/// Wraps a [`Paginator`] so the [`crate::crawl::crawler::Crawler`] can optionally call it during
/// its link-discovery step. `Arc` so it is cheap to share across worker threads.
pub type PaginatorHandle = Arc<Paginator>;

/// Generates all page URLs for a `PageNumber`-scheme config without network access. Useful for
/// pre-seeding a frontier or for offline testing.
pub fn generate_page_urls(url_template: &str, start_page: usize, max_pages: usize) -> Vec<String> {
    (start_page..=max_pages)
        .map(|n| url_template.replace("{page}", &n.to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crawl::frontier::MemoryFrontier;
    use crate::parser::document::DomTree;
    use std::sync::Arc;

    fn empty_dom() -> Arc<DomTree> {
        Arc::new(DomTree::default())
    }

    #[test]
    fn generate_page_urls_produces_numbered_sequence() {
        let urls = generate_page_urls("https://example.com/blog?page={page}", 1, 5);
        assert_eq!(urls.len(), 5);
        assert_eq!(urls[0], "https://example.com/blog?page=1");
        assert_eq!(urls[4], "https://example.com/blog?page=5");
    }

    #[test]
    fn page_number_paginator_enqueues_sequentially() {
        let config = PaginationConfig::page_number("https://example.com/p/{page}", 1, 3);
        let paginator = Paginator::new(config);
        let frontier = Arc::new(MemoryFrontier::new()) as Arc<dyn Frontier>;

        // Page 1 → enqueues page 1 URL, returns true
        let r1 = paginator.enqueue_next("https://example.com/p/0", &empty_dom(), &frontier).unwrap();
        assert!(r1);
        // Page 2
        let r2 = paginator.enqueue_next("https://example.com/p/1", &empty_dom(), &frontier).unwrap();
        assert!(r2);
        // Page 3: still within max_pages
        let r3 = paginator.enqueue_next("https://example.com/p/2", &empty_dom(), &frontier).unwrap();
        assert!(r3);
        // Page 4: past max_pages (3)
        let r4 = paginator.enqueue_next("https://example.com/p/3", &empty_dom(), &frontier).unwrap();
        assert!(!r4, "should stop after max_pages");

        assert_eq!(frontier.pending_len(), 3);
    }

    #[test]
    fn url_pattern_increments_page_number_in_url() {
        let config = PaginationConfig::url_pattern(r"/page/(\d+)/", 5);
        let paginator = Paginator::new(config);

        let next = paginator
            .next_url("https://example.com/blog/page/2/more", &empty_dom())
            .unwrap();
        assert_eq!(next.as_deref(), Some("https://example.com/blog/page/3/more"));
    }

    #[test]
    fn url_pattern_stops_at_max_page() {
        let config = PaginationConfig::url_pattern(r"/page/(\d+)/", 3);
        let paginator = Paginator::new(config);

        // Current page 3 → next would be 4 > max_page, so None.
        let next = paginator
            .next_url("https://example.com/blog/page/3/", &empty_dom())
            .unwrap();
        assert!(next.is_none());
    }

    #[test]
    fn url_pattern_no_match_returns_none() {
        let config = PaginationConfig::url_pattern(r"/page/(\d+)/", 10);
        let paginator = Paginator::new(config);
        let next = paginator
            .next_url("https://example.com/blog", &empty_dom())
            .unwrap();
        assert!(next.is_none());
    }
}
