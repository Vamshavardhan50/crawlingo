//! Phase 3 — Real browser fetch tier via Chrome DevTools Protocol.
//!
//! [`BrowserManager`] owns a single Chromium process and caps concurrent page activity via a
//! [`tokio::sync::Semaphore`]. It implements the same fetch interface as the HTTP tiers and slots
//! cleanly into the existing [`super::fetcher::FetchManager`] without touching anything in the
//! crawler, selector, or extraction layers.
//!
//! # Feature gate
//! Only compiled when `--features browser` is passed. The stub in `fetcher.rs` is used otherwise.
//!
//! # Environment
//! Set `CRAWLINGO_CHROME_PATH` to the absolute path of a Chrome/Chromium binary. Falls back to
//! common OS locations. Tests are skipped if no binary is found.

#![cfg(feature = "browser")]

use crate::engine::fetcher::{FetchRequest, NormalizedResponse, ResponseTimings};
use crate::error::{CrawlingoError, Result};
use bytes::Bytes;
use chromiumoxide::browser::{Browser, BrowserConfig as ChromeBuildConfig};
use chromiumoxide::cdp::browser_protocol::network::{
    CookieParam, EnableParams as NetworkEnableParams, SetExtraHttpHeadersParams,
};
use chromiumoxide::page::Page;
use futures::StreamExt;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;

// ─── Public configuration ─────────────────────────────────────────────────────

/// Configuration for the headless Chromium process managed by [`BrowserManager`].
#[derive(Debug, Clone)]
pub struct BrowserConfig {
    /// Absolute path to the Chrome/Chromium binary.
    /// Defaults to `CRAWLINGO_CHROME_PATH` env var, then common OS locations.
    pub chrome_path: Option<PathBuf>,
    /// Run in headless mode. Default: `true`.
    pub headless: bool,
    /// Maximum number of browser tabs open simultaneously. Default: `8`.
    pub max_concurrent_pages: usize,
    /// Per-page navigation timeout. Default: `30s`.
    pub page_timeout: Duration,
    /// Viewport width. Default: `1920`.
    pub viewport_width: u32,
    /// Viewport height. Default: `1080`.
    pub viewport_height: u32,
    /// Custom user-agent string. `None` uses the Chrome default.
    pub user_agent: Option<String>,
    /// Additional args forwarded verbatim to the Chrome launch command.
    pub extra_args: Vec<String>,
}

impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            chrome_path: std::env::var("CRAWLINGO_CHROME_PATH")
                .ok()
                .map(PathBuf::from),
            headless: true,
            max_concurrent_pages: 8,
            page_timeout: Duration::from_secs(30),
            viewport_width: 1920,
            viewport_height: 1080,
            user_agent: None,
            extra_args: Vec::new(),
        }
    }
}

impl BrowserConfig {
    /// Constructs config from environment. Suitable for [`FetchManager::new`].
    pub fn from_env() -> Self {
        Self::default()
    }

    /// Returns the Chrome binary path to use, falling back to OS defaults.
    pub fn resolved_chrome_path(&self) -> Result<PathBuf> {
        if let Some(ref p) = self.chrome_path {
            return Ok(p.clone());
        }

        let candidates: &[&str] = if cfg!(target_os = "windows") {
            &[
                r"C:\Program Files\Google\Chrome\Application\chrome.exe",
                r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
                r"C:\Program Files\Chromium\Application\chrome.exe",
                r"C:\Users\Lenovo\AppData\Local\Google\Chrome\Application\chrome.exe",
            ]
        } else if cfg!(target_os = "macos") {
            &[
                "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
                "/Applications/Chromium.app/Contents/MacOS/Chromium",
            ]
        } else {
            &[
                "/usr/bin/google-chrome",
                "/usr/bin/google-chrome-stable",
                "/usr/bin/chromium-browser",
                "/usr/bin/chromium",
                "/snap/bin/chromium",
            ]
        };

        for candidate in candidates {
            let p = PathBuf::from(candidate);
            if p.exists() {
                return Ok(p);
            }
        }

        Err(CrawlingoError::BrowserError(
            "Chrome/Chromium binary not found. \
             Set CRAWLINGO_CHROME_PATH to the binary path."
                .to_string(),
        ))
    }
}

// ─── BrowserManager ───────────────────────────────────────────────────────────

/// Manages the lifecycle of a single Chromium process and caps concurrent page activity.
///
/// Share as `Arc<BrowserManager>` across all fetch workers. The browser is launched once;
/// each `fetch_page` call opens a tab, captures the rendered HTML, then closes the tab.
/// A [`Semaphore`] hard-caps concurrent tabs to `config.max_concurrent_pages`.
pub struct BrowserManager {
    // Mutex because Browser::new_page takes &self but Browser itself is not Clone.
    browser: tokio::sync::Mutex<Option<Browser>>,
    semaphore: Arc<Semaphore>,
    config: BrowserConfig,
}

impl BrowserManager {
    /// Launches a Chromium process and returns a shared manager.
    pub async fn new(config: BrowserConfig) -> Result<Arc<Self>> {
        let chrome_path = config.resolved_chrome_path()?;

        tracing::info!(
            path = %chrome_path.display(),
            headless = config.headless,
            max_tabs = config.max_concurrent_pages,
            "Launching Chromium process"
        );

        let mut builder = ChromeBuildConfig::builder()
            .chrome_executable(chrome_path)
            .window_size(config.viewport_width, config.viewport_height)
            .arg("--no-sandbox")
            .arg("--disable-dev-shm-usage")
            .arg("--disable-gpu")
            .arg("--disable-extensions")
            .arg("--disable-background-networking")
            .arg("--disable-sync")
            .arg("--metrics-recording-only")
            .arg("--disable-default-apps")
            .arg("--mute-audio")
            .arg("--no-first-run")
            .arg("--no-default-browser-check");

        if config.headless {
            builder = builder.arg("--headless=new");
        }

        if let Some(ref ua) = config.user_agent {
            builder = builder.arg(format!("--user-agent={ua}"));
        }

        for arg in &config.extra_args {
            builder = builder.arg(arg.as_str());
        }

        let chrome_config = builder
            .build()
            .map_err(|e| CrawlingoError::BrowserError(format!("Chrome config error: {e}")))?;

        let (browser, mut handler) = Browser::launch(chrome_config)
            .await
            .map_err(|e| CrawlingoError::BrowserError(format!("Failed to launch Chrome: {e}")))?;

        // Drive the CDP event handler loop — required for all browser operations.
        tokio::spawn(async move { while handler.next().await.is_some() {} });

        tracing::info!("Chromium process ready");

        Ok(Arc::new(Self {
            browser: tokio::sync::Mutex::new(Some(browser)),
            semaphore: Arc::new(Semaphore::new(config.max_concurrent_pages)),
            config,
        }))
    }

    /// Fetches a URL using a fresh browser tab, returning a normalized response.
    ///
    /// Injects cookies and headers from `request` before navigation.
    /// Returns [`CrawlingoError::BrowserError`] on failure — never falls back to HTTP.
    pub async fn fetch_page(&self, request: &FetchRequest) -> Result<NormalizedResponse> {
        // Acquire semaphore slot — blocks if max_concurrent_pages are in-flight.
        let _permit =
            self.semaphore.acquire().await.map_err(|_| {
                CrawlingoError::BrowserError("Browser semaphore closed".to_string())
            })?;

        let page = self.open_page(request).await?;

        // Wait for navigation to complete, with timeout.
        let wait_result =
            tokio::time::timeout(self.config.page_timeout, page.wait_for_navigation()).await;

        match wait_result {
            Err(_elapsed) => {
                tracing::warn!(url = %request.url, "Browser navigation timed out; using partial content");
            }
            Ok(Err(e)) => {
                // Close the page before returning to avoid leaking the tab.
                let _ = page.close().await;
                return Err(CrawlingoError::BrowserError(format!(
                    "Navigation error for '{}': {e}",
                    request.url
                )));
            }
            Ok(Ok(_)) => {}
        }

        // Get fully-rendered HTML (post-JS execution).
        let html = page.content().await.map_err(|e| {
            CrawlingoError::BrowserError(format!("Failed to get page content: {e}"))
        })?;

        // Collect cookies set by the page.
        let cookies = self.collect_page_cookies(&page).await;

        // Final URL after any redirects.
        let final_url = page
            .url()
            .await
            .ok()
            .flatten()
            .unwrap_or_else(|| request.url.clone());

        // Close tab — releases its resources in the browser process.
        let _ = page.close().await;
        // Semaphore permit auto-released via `_permit` drop.

        Ok(NormalizedResponse {
            url: final_url,
            // CDP's `Page.navigate` doesn't cleanly surface the HTTP status to Rust post-load.
            // 200 is the correct assumption for a successfully loaded page.
            status: 200,
            headers: HashMap::new(),
            cookies,
            body: Bytes::from(html.into_bytes()),
            content_type: "text/html; charset=utf-8".to_string(),
            encoding: "utf-8".to_string(),
            timings: ResponseTimings::default(),
        })
    }

    /// Opens a new blank page, injects cookies/headers, and begins navigation.
    async fn open_page(&self, request: &FetchRequest) -> Result<Page> {
        let guard = self.browser.lock().await;
        let browser = guard.as_ref().ok_or_else(|| {
            CrawlingoError::BrowserError("BrowserManager has been shut down".to_string())
        })?;

        // Open a blank tab — using `about:blank` avoids any initial network activity.
        let page = browser
            .new_page("about:blank")
            .await
            .map_err(|e| CrawlingoError::BrowserError(format!("Failed to open tab: {e}")))?;

        // Enable network domain for cookie/header operations.
        page.execute(NetworkEnableParams::default())
            .await
            .map_err(|e| CrawlingoError::BrowserError(format!("Network.enable failed: {e}")))?;

        // Inject request cookies via CDP Network.setCookies.
        if !request.cookies.is_empty() {
            let parsed = url::Url::parse(&request.url).map_err(|e| {
                CrawlingoError::BrowserError(format!("Invalid URL '{}': {e}", request.url))
            })?;
            let domain = parsed.host_str().unwrap_or("").to_string();

            let cookie_params: Vec<CookieParam> = request
                .cookies
                .iter()
                .map(|(k, v)| CookieParam {
                    name: k.clone(),
                    value: v.clone(),
                    url: None,
                    domain: Some(domain.clone()),
                    path: Some("/".to_string()),
                    secure: None,
                    http_only: None,
                    same_site: None,
                    expires: None,
                    priority: None,
                    same_party: None,
                    source_scheme: None,
                    source_port: None,
                    partition_key: None,
                })
                .collect();

            page.set_cookies(cookie_params).await.map_err(|e| {
                CrawlingoError::BrowserError(format!("Network.setCookies failed: {e}"))
            })?;
        }

        // Inject extra request headers via CDP Network.setExtraHTTPHeaders.
        if !request.headers.is_empty() {
            let headers_obj: serde_json::Map<String, serde_json::Value> = request
                .headers
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                .collect();

            page.execute(SetExtraHttpHeadersParams::new(
                chromiumoxide::cdp::browser_protocol::network::Headers::new(
                    serde_json::Value::Object(headers_obj),
                ),
            ))
            .await
            .map_err(|e| {
                CrawlingoError::BrowserError(format!("Network.setExtraHTTPHeaders failed: {e}"))
            })?;
        }

        // Navigate to target URL.
        page.goto(request.url.clone()).await.map_err(|e| {
            CrawlingoError::BrowserError(format!("Navigation to '{}' failed: {e}", request.url))
        })?;

        Ok(page)
    }

    /// Collects all cookies currently on the page.
    async fn collect_page_cookies(&self, page: &Page) -> HashMap<String, String> {
        match page.get_cookies().await {
            Ok(cookies) => cookies
                .into_iter()
                .map(|c| (c.name.clone(), c.value.clone()))
                .collect(),
            Err(_) => HashMap::new(),
        }
    }

    /// Gracefully closes the Chromium process. Idempotent — subsequent calls are no-ops.
    pub async fn shutdown(&self) {
        let mut guard = self.browser.lock().await;
        if let Some(mut browser) = guard.take() {
            tracing::info!("Shutting down Chromium process");
            let _ = browser.close().await;
            let _ = browser.wait().await;
        }
    }
}

impl Drop for BrowserManager {
    fn drop(&mut self) {
        // Best-effort warning — explicit `shutdown()` is preferred.
        if let Ok(guard) = self.browser.try_lock() {
            if guard.is_some() {
                tracing::warn!(
                    "BrowserManager dropped without explicit shutdown(). \
                     Call shutdown() before drop for clean Chromium process teardown."
                );
            }
        }
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::fetcher::{FetchRequest, FetcherTier};
    use std::collections::HashMap;

    /// Skip helper — returns `None` if Chrome binary isn't available.
    fn chrome_binary() -> Option<PathBuf> {
        BrowserConfig::from_env().resolved_chrome_path().ok()
    }

    fn browser_request(url: &str) -> FetchRequest {
        FetchRequest {
            url: url.to_string(),
            tier: FetcherTier::Browser,
            browser_profile: None,
            headers: HashMap::new(),
            cookies: HashMap::new(),
            proxy: None,
            timeout: Duration::from_secs(30),
            retries: 0,
        }
    }

    #[tokio::test]
    async fn test_browser_config_default_from_env() {
        // BrowserConfig::from_env() should not panic regardless of env state.
        let cfg = BrowserConfig::from_env();
        assert!(cfg.headless);
        assert_eq!(cfg.max_concurrent_pages, 8);
        assert_eq!(cfg.page_timeout, Duration::from_secs(30));
    }

    #[tokio::test]
    async fn test_browser_config_path_from_env() {
        // When CRAWLINGO_CHROME_PATH is set, it should be picked up.
        std::env::set_var("CRAWLINGO_CHROME_PATH", "/tmp/fake-chrome");
        let cfg = BrowserConfig::from_env();
        assert_eq!(cfg.chrome_path, Some(PathBuf::from("/tmp/fake-chrome")));
        std::env::remove_var("CRAWLINGO_CHROME_PATH");
    }

    #[tokio::test]
    async fn test_browser_manager_launches_and_fetches() {
        let Some(chrome) = chrome_binary() else {
            println!("SKIP: Chrome binary not found. Set CRAWLINGO_CHROME_PATH.");
            return;
        };

        let config = BrowserConfig {
            chrome_path: Some(chrome),
            headless: true,
            max_concurrent_pages: 2,
            page_timeout: Duration::from_secs(20),
            ..Default::default()
        };

        let manager = BrowserManager::new(config)
            .await
            .expect("BrowserManager should launch");

        // Use a data: URI so this test has zero network dependency.
        let req = browser_request("data:text/html,<h1 id='test'>BrowserOK</h1>");
        let resp = manager
            .fetch_page(&req)
            .await
            .expect("fetch_page should succeed");

        assert_eq!(resp.status, 200);
        let body = String::from_utf8_lossy(&resp.body);
        assert!(
            body.contains("BrowserOK"),
            "Expected 'BrowserOK' in body, got: {body}"
        );

        manager.shutdown().await;
    }

    #[tokio::test]
    async fn test_browser_fetcher_concurrent_pages() {
        let Some(chrome) = chrome_binary() else {
            println!("SKIP: Chrome binary not found. Set CRAWLINGO_CHROME_PATH.");
            return;
        };

        let config = BrowserConfig {
            chrome_path: Some(chrome),
            headless: true,
            max_concurrent_pages: 4,
            page_timeout: Duration::from_secs(15),
            ..Default::default()
        };

        let manager = BrowserManager::new(config)
            .await
            .expect("BrowserManager should launch");

        let manager = Arc::clone(&manager);
        let mut tasks = Vec::new();

        for i in 0..4 {
            let m = Arc::clone(&manager);
            tasks.push(tokio::spawn(async move {
                let req = browser_request(&format!("data:text/html,<h1>Page{i}</h1>"));
                m.fetch_page(&req).await
            }));
        }

        for task in tasks {
            let result = task.await.expect("Task should not panic");
            assert!(result.is_ok(), "Concurrent fetch failed: {result:?}");
        }

        manager.shutdown().await;
    }

    #[tokio::test]
    async fn test_browser_manager_process_reused() {
        let Some(chrome) = chrome_binary() else {
            println!("SKIP: Chrome binary not found.");
            return;
        };

        let config = BrowserConfig {
            chrome_path: Some(chrome),
            headless: true,
            max_concurrent_pages: 2,
            page_timeout: Duration::from_secs(15),
            ..Default::default()
        };

        let manager = BrowserManager::new(config)
            .await
            .expect("BrowserManager should launch");

        // Two sequential fetches — same manager → same browser process.
        let r1 = manager
            .fetch_page(&browser_request("data:text/html,<p>first</p>"))
            .await
            .expect("first fetch");
        let r2 = manager
            .fetch_page(&browser_request("data:text/html,<p>second</p>"))
            .await
            .expect("second fetch");

        assert!(String::from_utf8_lossy(&r1.body).contains("first"));
        assert!(String::from_utf8_lossy(&r2.body).contains("second"));

        manager.shutdown().await;
    }

    #[tokio::test]
    async fn test_browser_fetcher_cookie_injection() {
        let Some(chrome) = chrome_binary() else {
            println!("SKIP: Chrome binary not found.");
            return;
        };

        let config = BrowserConfig {
            chrome_path: Some(chrome),
            headless: true,
            max_concurrent_pages: 1,
            page_timeout: Duration::from_secs(15),
            ..Default::default()
        };

        let manager = BrowserManager::new(config)
            .await
            .expect("BrowserManager should launch");

        // Page that reads document.cookie via JS and writes it to the DOM.
        let html = r#"data:text/html,<html><body><script>
            document.body.innerHTML = '<p id="c">' + document.cookie + '</p>';
        </script></body></html>"#;

        let mut req = browser_request(html);
        req.cookies
            .insert("sessionid".to_string(), "abc123".to_string());

        // Note: cookies injected via CDP.Network.setCookies don't work for data: URIs (no domain).
        // This test verifies the injection machinery doesn't error, not the cookie value.
        let resp = manager.fetch_page(&req).await;
        assert!(
            resp.is_ok(),
            "Cookie injection should not cause a hard error: {resp:?}"
        );

        manager.shutdown().await;
    }
}
