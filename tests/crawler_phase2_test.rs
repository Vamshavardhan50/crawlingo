use crawlingo::crawl::crawler::Crawler;
use crawlingo::crawl::frontier::{Frontier, PersistentFrontier};
use crawlingo::crawl::robots::RobotsRules;
use crawlingo::dataset::builder::DatasetField;
use crawlingo::engine::session::Session;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

async fn start_mock_server() -> (SocketAddr, tokio::sync::oneshot::Sender<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (tx, mut rx) = tokio::sync::oneshot::channel::<()>();

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = &mut rx => break,
                Ok((mut socket, _)) = listener.accept() => {
                    tokio::spawn(async move {
                        let mut buf = [0u8; 2048];
                        if let Ok(n) = socket.read(&mut buf).await {
                            let request = String::from_utf8_lossy(&buf[..n]);

                            if request.contains("GET /robots.txt") {
                                let body = "User-agent: *\nDisallow: /disallowed/\n";
                                let resp = format!(
                                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                                    body.len(), body
                                );
                                let _ = socket.write_all(resp.as_bytes()).await;
                            } else if request.contains("GET /disallowed/") {
                                let body = "<html><body><h1>Disallowed</h1></body></html>";
                                let resp = format!(
                                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                                    body.len(), body
                                );
                                let _ = socket.write_all(resp.as_bytes()).await;
                            } else if request.contains("GET /page1") {
                                let body = format!(
                                    "<html><body><h1>Page 1</h1><a href='http://{}/page2'>Link 2</a><a href='http://{}/disallowed/secret'>Secret</a></body></html>",
                                    addr, addr
                                );
                                let resp = format!(
                                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                                    body.len(), body
                                );
                                let _ = socket.write_all(resp.as_bytes()).await;
                            } else if request.contains("GET /page2") {
                                let body = format!(
                                    "<html><body><h1>Page 2</h1><a href='http://{}/page3'>Link 3</a></body></html>",
                                    addr
                                );
                                let resp = format!(
                                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                                    body.len(), body
                                );
                                let _ = socket.write_all(resp.as_bytes()).await;
                            } else if request.contains("GET /page3") {
                                let body = "<html><body><h1>Page 3</h1></body></html>";
                                let resp = format!(
                                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                                    body.len(), body
                                );
                                let _ = socket.write_all(resp.as_bytes()).await;
                            } else if request.contains("POST /webhook") {
                                let resp = "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nOK";
                                let _ = socket.write_all(resp.as_bytes()).await;
                            } else {
                                let body = "<html><body><h1>Index</h1></body></html>";
                                let resp = format!(
                                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                                    body.len(), body
                                );
                                let _ = socket.write_all(resp.as_bytes()).await;
                            }
                        }
                    });
                }
            }
        }
    });

    (addr, tx)
}

#[tokio::test]
async fn test_phase2_frontier_canonical_deduplication_and_crash_recovery() {
    let dir = tempfile::tempdir().unwrap();

    // 1. Enqueue URLs with fragment and check deduplication
    {
        let frontier = PersistentFrontier::open(dir.path()).unwrap();
        frontier.enqueue("http://example.com/page#sec1".to_string(), 0);
        frontier.enqueue("http://example.com/page#sec2".to_string(), 0);
        assert_eq!(frontier.pending_len(), 1);

        let (url, depth) = frontier.dequeue().unwrap();
        assert_eq!(url, "http://example.com/page");
        assert_eq!(depth, 0);

        // Mark in-flight to simulate process crash mid-fetch
        frontier.mark_in_flight(&url, depth);
        assert_eq!(frontier.in_flight_len(), 1);
    }

    // 2. Reopen DB — crashed task must be automatically recovered into pending
    let reopened = PersistentFrontier::open(dir.path()).unwrap();
    assert_eq!(reopened.pending_len(), 1);
    assert_eq!(reopened.in_flight_len(), 0);

    let (url, _) = reopened.dequeue().unwrap();
    assert_eq!(url, "http://example.com/page");
}

#[tokio::test]
async fn test_phase2_robots_txt_allow_disallow_rules() {
    let content = r#"
User-agent: *
Disallow: /private/
Allow: /private/public/
"#;
    let rules = RobotsRules::parse(content);
    assert!(!rules.is_allowed("CrawlingoBot", "/private/secret"));
    assert!(rules.is_allowed("CrawlingoBot", "/private/public/page"));
    assert!(rules.is_allowed("CrawlingoBot", "/public/index"));
}

#[tokio::test]
async fn test_phase2_crawler_depth_concurrency_robots_and_metrics() {
    let (addr, _shutdown) = start_mock_server().await;
    let session = Arc::new(Session::new());

    let start_url = format!("http://{}/page1", addr);

    let crawler = Crawler::new(&start_url, session)
        .max_depth(2)
        .concurrency(4)
        .follow("a")
        .respect_robots(true)
        .field(DatasetField {
            name: "title".to_string(),
            selector: "h1".to_string(),
            selector_type: "css".to_string(),
            #[cfg(feature = "python")]
            transform: None,
            default: None,
            extract_type: crawlingo::extraction::ExtractionType::Text,
        });

    let result = crawler.crawl_async().await.unwrap();

    assert!(result.visited_count >= 3);
    assert!(result.successful_count >= 3);
    assert_eq!(result.skipped_count, 1); // /disallowed/secret skipped by robots.txt
    assert!(!result.results.is_empty());
}

#[tokio::test]
async fn test_phase2_crawler_cancellation() {
    let (addr, _shutdown) = start_mock_server().await;
    let session = Arc::new(Session::new());
    let start_url = format!("http://{}/page1", addr);

    let crawler = Crawler::new(&start_url, session).concurrency(2);
    crawler.cancel(); // Cancel immediately

    let result = crawler.crawl_async().await.unwrap();
    // Cancelled crawler exits cleanly without fetching pending queue
    assert_eq!(result.successful_count, 0);
}
