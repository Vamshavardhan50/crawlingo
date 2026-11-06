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
    }
}

#[tokio::test]
async fn stealth_profiles_emit_different_tls_cipher_offers() {
    async fn capture(profile: &str) -> Vec<u16> {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let receiver = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut header = [0; 5];
            socket.read_exact(&mut header).await.unwrap();
            assert_eq!(header[0], 22, "expected TLS handshake record");
            let mut hello = vec![0; u16::from_be_bytes([header[3], header[4]]) as usize];
            socket.read_exact(&mut hello).await.unwrap();
            assert_eq!(hello[0], 1, "expected ClientHello");
            // Handshake header, legacy version, 32 random bytes, then the session ID.
            let start = 39 + usize::from(hello[38]);
            let size = u16::from_be_bytes([hello[start], hello[start + 1]]) as usize;
            let (ciphers, remainder) = hello[start + 2..start + 2 + size].as_chunks::<2>();
            assert!(remainder.is_empty(), "cipher suite list must contain pairs");
            ciphers
                .iter()
                // Ignore randomized GREASE entries when comparing real cipher suites.
                .filter(|pair| !(pair[0] == pair[1] && pair[0] & 15 == 10))
                .map(|pair| u16::from_be_bytes([pair[0], pair[1]]))
                .collect::<Vec<_>>()
        });
        let fetcher = HttpFetcher::new(ConnectionPoolConfig::default());
        let req = request(
            &format!("https://{address}/"),
            FetcherTier::Stealthy,
            Some(profile),
            None,
        );
        let (_, captured) = tokio::time::timeout(Duration::from_secs(15), async {
            tokio::join!(fetcher.fetch(&req), receiver)
        })
        .await
        .unwrap();
        captured.unwrap()
    }
    let chrome = capture("chrome").await;
    let firefox = capture("firefox").await;
    assert!(!chrome.is_empty() && !firefox.is_empty());
    assert_ne!(
        chrome, firefox,
        "configured profiles must affect the wire handshake"
    );
}

#[tokio::test]
async fn total_fetch_duration_includes_slow_body_delivery() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut buffer = [0u8; 4096];
        assert!(socket.read(&mut buffer).await.unwrap() > 0);
        socket
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\nConnection: close\r\n\r\n")
            .await
            .unwrap();
        tokio::time::sleep(Duration::from_millis(120)).await;
        socket.write_all(b"hello").await.unwrap();
    });
    let fetcher = HttpFetcher::new(ConnectionPoolConfig::default());
    let response = fetcher
        .fetch(&request(
            &format!("http://{address}/"),
            FetcherTier::Standard,
            None,
            None,
        ))
        .await
        .unwrap();
    assert_eq!(response.body.as_ref(), b"hello");
    assert!(response.timings.total_duration >= Duration::from_millis(100));
    server.await.unwrap();
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
        .fetch(&request(
            "http://127.0.0.1:1/a",
            FetcherTier::Standard,
            None,
            None,
        ))
        .await;
    let _ = fetcher
        .fetch(&request(
            "http://127.0.0.1:1/b",
            FetcherTier::Stealthy,
            None,
            None,
        ))
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
        .fetch(&request(
            "http://127.0.0.1:1/a",
            FetcherTier::Standard,
            None,
            None,
        ))
        .await;
    let _ = fetcher
        .fetch(&request(
            "http://127.0.0.1:1/b",
            FetcherTier::Stealthy,
            None,
            None,
        ))
        .await;

    assert!(
        fetcher.cached_client_count().await <= 1,
        "configured max_clients bound must cap the cache"
    );
}
