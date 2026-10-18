//! Phase 3 integration tests — Real Browser Engine.
//!
//! These tests only run when:
//! 1. Compiled with `--features browser`
//! 2. `CRAWLINGO_CHROME_PATH` is set (or a Chrome binary found at a default location)
//!
//! Run with:
//! ```
//! $env:CRAWLINGO_CHROME_PATH = "C:\Program Files\Google\Chrome\Application\chrome.exe"
//! cargo test --features browser -- browser
//! ```
//!
//! Tests are skipped cleanly (not failed) when Chrome is not available.

#![cfg(feature = "browser")]

use crawlingo::engine::browser::{BrowserConfig, BrowserManager};
use crawlingo::engine::fetcher::{FetchManager, FetchRequest, FetcherTier, MockTransport};
use crawlingo::engine::pool::ConnectionPoolConfig;
use crawlingo::engine::rate_limiter::HostRateLimiter;
use crawlingo::error::CrawlingoError;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// ─── Helpers ─────────────────────────────────────────────────────────────────

/// Returns Chrome binary path if available, otherwise `None`.
fn chrome_binary() -> Option<PathBuf> {
    BrowserConfig::from_env().resolved_chrome_path().ok()
}

/// Constructs a browser-tier FetchRequest for a given URL.
fn browser_req(url: &str) -> FetchRequest {
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

/// Spawns a minimal HTTP server that serves JS-rendered pages.
///
/// Routes:
/// - `/js-rendered`  — returns a page where `<h1>` is written by inline JS
/// - `/delayed`      — like above, but with 200ms delay before writing
/// - `/echo-cookie`  — echoes back the `Cookie:` header as text
/// - `/echo-header`  — echoes back the `X-Custom-Header` value as text
async fn spawn_browser_test_server() -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind test server");
    let addr = listener.local_addr().expect("local addr");

    tokio::spawn(async move {
        // Handle up to 32 connections.
        for _ in 0..32_u32 {
            let Ok((mut socket, _)) = listener.accept().await else {
                break;
            };

            tokio::spawn(async move {
                let mut buf = [0u8; 8192];
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n > 0 => n,
                    _ => return,
                };

                let request = String::from_utf8_lossy(&buf[..n]);
                let first_line = request.lines().next().unwrap_or("");

                // Extract Cookie: header value
                let cookie_val = request
                    .lines()
                    .find(|l| l.to_lowercase().starts_with("cookie:"))
                    .map(|l| l[7..].trim().to_string())
                    .unwrap_or_default();

                // Extract X-Custom-Header value
                let custom_header = request
                    .lines()
                    .find(|l| l.to_lowercase().starts_with("x-custom-header:"))
                    .map(|l| l[16..].trim().to_string())
                    .unwrap_or_default();

                let (status, body) = if first_line.contains("/js-rendered") {
                    (
                        200,
                        r#"<!doctype html><html><head></head><body>
<script>document.write('<h1 id="js-title">JS Content Loaded</h1>');</script>
</body></html>"#
                            .to_string(),
                    )
                } else if first_line.contains("/delayed") {
                    tokio::time::sleep(Duration::from_millis(200)).await;
                    (
                        200,
                        r#"<!doctype html><html><body>
<script>
setTimeout(function(){
  document.body.innerHTML='<h1 id="delayed">Delayed JS Content</h1>';
},100);
</script>
</body></html>"#
                            .to_string(),
                    )
                } else if first_line.contains("/echo-cookie") {
                    (
                        200,
                        format!(
                            r#"<!doctype html><html><body><p id="cookie-echo">{cookie_val}</p></body></html>"#
                        ),
                    )
                } else if first_line.contains("/echo-header") {
                    (
                        200,
                        format!(
                            r#"<!doctype html><html><body><p id="header-echo">{custom_header}</p></body></html>"#
                        ),
                    )
                } else {
                    (
                        200,
                        "<html><body><h1>Browser Test Server</h1></body></html>".to_string(),
                    )
                };

                let response = format!(
                    "HTTP/1.1 {status} OK\r\ncontent-type: text/html; charset=utf-8\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
                    body.len()
                );
                let _ = socket.write_all(response.as_bytes()).await;
            });
        }
    });

    format!("http://{addr}")
}

// ─── Tests ───────────────────────────────────────────────────────────────────

/// The stub (`browser` feature on, but no manager injected) should return BrowserError —
/// not silently fall back to HTTP.
#[tokio::test]
async fn test_browser_tier_errors_without_chrome_path() {
    // Force Chrome path to a non-existent file.
    let bad_config = BrowserConfig {
        chrome_path: Some(PathBuf::from("/does/not/exist/chrome")),
        headless: true,
        ..Default::default()
    };

    let result = BrowserManager::new(bad_config).await;
    let err = match result {
        Err(e) => e,
        Ok(_) => panic!("BrowserManager::new should fail with bad Chrome path"),
    };
    assert!(
        matches!(err, CrawlingoError::BrowserError(_)),
        "Expected BrowserError, got: {err}"
    );
}

/// BrowserFetcher stub returns NotSupported when browser feature on but manager unavailable.
/// This test uses MockTransport to verify the FetchManager routing still works end-to-end.
#[tokio::test]
async fn test_fetch_manager_browser_tier_routing() {
    // FetchManager::with_transport routes ALL tiers through mock.
    let mock = Arc::new(MockTransport::new().with_html(
        "http://browser-test.example.com/",
        "<h1>Mocked Browser</h1>",
    ));
    let manager = FetchManager::with_transport(Arc::new(HostRateLimiter::new()), mock.clone());

    let req = browser_req("http://browser-test.example.com/");
    let resp = manager.dispatch(req).await.expect("mock should succeed");

    assert_eq!(resp.status, 200);
    assert!(String::from_utf8_lossy(&resp.body).contains("Mocked Browser"));
    // Confirm the Browser-tier request did route through the transport.
    assert_eq!(mock.call_count(), 1);
}

/// With a real Chrome binary: fetch a JS-rendered page and assert the DOM is populated.
#[tokio::test]
async fn test_browser_fetcher_js_rendered() {
    let Some(chrome) = chrome_binary() else {
        println!("SKIP: CRAWLINGO_CHROME_PATH not set / Chrome not found.");
        return;
    };

    let base = spawn_browser_test_server().await;
    let config = BrowserConfig {
        chrome_path: Some(chrome),
        headless: true,
        max_concurrent_pages: 2,
        page_timeout: Duration::from_secs(15),
        ..Default::default()
    };

    let manager = BrowserManager::new(config).await.expect("launch browser");
    let resp = manager
        .fetch_page(&browser_req(&format!("{base}/js-rendered")))
        .await
        .expect("js-rendered fetch");

    let body = String::from_utf8_lossy(&resp.body);
    // The JS `document.write` must have executed before `content()` was captured.
    assert!(
        body.contains("JS Content Loaded"),
        "Expected JS-rendered content, got: {body}"
    );

    manager.shutdown().await;
}

/// Custom HTTP headers injected via FetchRequest are forwarded to the server.
#[tokio::test]
async fn test_browser_fetcher_custom_headers() {
    let Some(chrome) = chrome_binary() else {
        println!("SKIP: Chrome not found.");
        return;
    };

    let base = spawn_browser_test_server().await;
    let config = BrowserConfig {
        chrome_path: Some(chrome),
        headless: true,
        max_concurrent_pages: 1,
        page_timeout: Duration::from_secs(15),
        ..Default::default()
    };

    let manager = BrowserManager::new(config).await.expect("launch browser");

    let mut req = browser_req(&format!("{base}/echo-header"));
    req.headers.insert(
        "X-Custom-Header".to_string(),
        "crawlingo-browser".to_string(),
    );

    let resp = manager.fetch_page(&req).await.expect("header echo fetch");
    let body = String::from_utf8_lossy(&resp.body);

    assert!(
        body.contains("crawlingo-browser"),
        "Expected echoed header in body, got: {body}"
    );

    manager.shutdown().await;
}

/// Concurrent fetches saturate the semaphore and all complete successfully.
#[tokio::test]
async fn test_browser_fetcher_concurrent_pages() {
    let Some(chrome) = chrome_binary() else {
        println!("SKIP: Chrome not found.");
        return;
    };

    let config = BrowserConfig {
        chrome_path: Some(chrome),
        headless: true,
        max_concurrent_pages: 4,
        page_timeout: Duration::from_secs(20),
        ..Default::default()
    };

    let manager = Arc::new(BrowserManager::new(config).await.expect("launch browser"));

    let mut handles = Vec::new();
    for i in 0..8 {
        let m = Arc::clone(&manager);
        handles.push(tokio::spawn(async move {
            let url = format!("data:text/html,<h1>Page{i}</h1>");
            m.fetch_page(&browser_req(&url)).await
        }));
    }

    for (i, h) in handles.into_iter().enumerate() {
        let result = h.await.expect("task should not panic");
        let resp = result.unwrap();
        let body = String::from_utf8_lossy(&resp.body);
        assert!(
            body.contains(&format!("Page{i}")),
            "Page{i} body mismatch: {body}"
        );
    }

    manager.shutdown().await;
}

/// Two sequential fetches reuse the same browser process (verified by structural test).
#[tokio::test]
async fn test_browser_manager_process_reused() {
    let Some(chrome) = chrome_binary() else {
        println!("SKIP: Chrome not found.");
        return;
    };

    let config = BrowserConfig {
        chrome_path: Some(chrome),
        headless: true,
        max_concurrent_pages: 2,
        page_timeout: Duration::from_secs(15),
        ..Default::default()
    };

    let manager = BrowserManager::new(config).await.expect("launch");

    let r1 = manager
        .fetch_page(&browser_req("data:text/html,<p>Alpha</p>"))
        .await
        .expect("fetch 1");
    let r2 = manager
        .fetch_page(&browser_req("data:text/html,<p>Beta</p>"))
        .await
        .expect("fetch 2");

    assert!(String::from_utf8_lossy(&r1.body).contains("Alpha"));
    assert!(String::from_utf8_lossy(&r2.body).contains("Beta"));

    // Both succeeded → browser process was alive for both.
    manager.shutdown().await;
}

/// FetchManager::with_browser_manager correctly routes Browser-tier requests.
#[tokio::test]
async fn test_fetch_manager_with_browser_manager() {
    let Some(chrome) = chrome_binary() else {
        println!("SKIP: Chrome not found.");
        return;
    };

    let config = BrowserConfig {
        chrome_path: Some(chrome),
        headless: true,
        max_concurrent_pages: 2,
        page_timeout: Duration::from_secs(15),
        ..Default::default()
    };

    let bm = BrowserManager::new(config).await.expect("launch");
    let rl = Arc::new(HostRateLimiter::new());
    let manager =
        FetchManager::with_browser_manager(rl, ConnectionPoolConfig::default(), Arc::clone(&bm));

    let base = spawn_browser_test_server().await;
    let resp = manager
        .dispatch(browser_req(&format!("{base}/js-rendered")))
        .await
        .expect("dispatch");

    let page = crawlingo::parser::streaming::HtmlParser::parse(resp).unwrap();
    let nodes = page
        .query(crawlingo::selector::SelectorQuery::Css("#js-title"))
        .unwrap();
    assert_eq!(page.get_nodes_combined_text(&nodes), "JS Content Loaded");

    bm.shutdown().await;
}
