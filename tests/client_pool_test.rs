//! Integration tests for the HTTP client connection pool (public-API level).
//!
//! These drive `HttpFetcher` through the public `Transport` trait and assert on the observable
//! client cache (`cached_client_count`). Before pooling, every request built a throwaway
//! `wreq::Client`, so `ConnectionPoolConfig` had no effect and connections were never reused.

use crawlingo::engine::fetcher::{FetchRequest, FetcherTier, HttpFetcher, Transport};
use crawlingo::engine::pool::ConnectionPoolConfig;
use std::collections::HashMap;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Spawns a minimal HTTP/1.1 server that answers `max_conns` connections with a fixed body.
async fn spawn_server(max_conns: usize, body: &'static str) -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        for _ in 0..max_conns {
            if let Ok((mut socket, _)) = listener.accept().await {
                let mut buf = [0_u8; 4096];
                if let Ok(n) = socket.read(&mut buf).await {
                    if n > 0 {
                        let resp = format!(
                            "HTTP/1.1 200 OK\r\ncontent-type: text/html; charset=utf-8\r\ncontent-length: {}\r\n\r\n{}",
                            body.len(),
                            body
                        );
                        let _ = socket.write_all(resp.as_bytes()).await;
                    }
                }
            }
        }
    });
    format!("http://{addr}")
}

fn request(
    url: &str,
    tier: FetcherTier,
    profile: Option<&str>,
    proxy: Option<&str>,
) -> FetchRequest {
    FetchRequest {
        url: url.to_string(),
        tier,
        browser_profile: profile.map(String::from),
        headers: HashMap::new(),
        cookies: HashMap::new(),
        proxy: proxy.map(String::from),
        timeout: Duration::from_secs(10),
        retries: 0,
        rate_limit_rps: 0.0,
    }
}

#[tokio::test]
async fn repeated_fetches_reuse_a_single_pooled_client() {
    // no_keepalive means each request opens a fresh TCP connection (so the tiny test server,
    // which handles one request per accepted socket, keeps working) — yet the *client* is still
    // cached and reused, which is the mechanism that enables connection pooling in production.
    let url = spawn_server(2, "<html><title>pool</title></html>").await;
    let fetcher = HttpFetcher::new(ConnectionPoolConfig::no_keepalive());

    let req = request(&url, FetcherTier::Standard, None, None);
    let first = fetcher.fetch(&req).await.expect("first fetch ok");
    let second = fetcher.fetch(&req).await.expect("second fetch ok");

    assert_eq!(first.status, 200);
    assert_eq!(second.status, 200);
    assert_eq!(&first.body[..], b"<html><title>pool</title></html>");
    assert_eq!(
        fetcher.cached_client_count().await,
        1,
        "both requests must share one cached client"
    );
}

#[tokio::test]
async fn distinct_configurations_are_cached_separately() {
    let fetcher = HttpFetcher::new(ConnectionPoolConfig::default());

    // Unreachable endpoints: the fetch fails on connect, but the client is cached beforehand,
    // which is exactly what we want to observe. retries=0 keeps this fast.
    let _ = fetcher
        .fetch(&request("http://127.0.0.1:1/a", FetcherTier::Standard, None, None))
        .await;
    let _ = fetcher
        .fetch(&request("http://127.0.0.1:1/b", FetcherTier::Stealthy, None, None))
        .await;
    let _ = fetcher
        .fetch(&request(
            "http://127.0.0.1:1/c",
            FetcherTier::Standard,
            None,
            Some("http://127.0.0.1:1"),
        ))
        .await;

    assert_eq!(
        fetcher.cached_client_count().await,
        3,
        "standard, stealth, and proxied transports are three distinct clients"
    );
}

#[tokio::test]
async fn max_clients_bound_is_enforced() {
    let cfg = ConnectionPoolConfig::default().with_max_clients(1);
    let fetcher = HttpFetcher::new(cfg);

    let _ = fetcher
        .fetch(&request("http://127.0.0.1:1/a", FetcherTier::Standard, None, None))
        .await;
    let _ = fetcher
        .fetch(&request("http://127.0.0.1:1/b", FetcherTier::Stealthy, None, None))
        .await;

    assert!(
        fetcher.cached_client_count().await <= 1,
        "configured max_clients bound must cap the cache"
    );
}
