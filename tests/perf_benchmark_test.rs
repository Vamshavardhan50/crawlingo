use crawlingo::crawl::crawler::Crawler;
use crawlingo::dataset::builder::DatasetField;
use crawlingo::engine::session::Session;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

async fn start_benchmark_server(
    total_pages: usize,
) -> (
    SocketAddr,
    tokio::sync::oneshot::Sender<()>,
    Arc<AtomicUsize>,
) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let (tx, mut rx) = tokio::sync::oneshot::channel::<()>();
    let request_count = Arc::new(AtomicUsize::new(0));
    let request_count_clone = request_count.clone();

    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = &mut rx => break,
                Ok((mut socket, _)) = listener.accept() => {
                    let req_count = request_count_clone.clone();
                    tokio::spawn(async move {
                        let mut buf = [0u8; 1024];
                        if let Ok(n) = socket.read(&mut buf).await {
                            if n == 0 { return; }
                            req_count.fetch_add(1, Ordering::Relaxed);

                            let mut body = "<html><body><h1>Benchmark Item</h1>".to_string();
                            if String::from_utf8_lossy(&buf[..n]).starts_with("GET /item/1 ") {
                                for id in 2..=total_pages { body.push_str(&format!("<a href='/item/{id}'>Item {id}</a>")); }
                            }
                            body.push_str("</body></html>");
                            let resp = format!(
                                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                                body.len(), body
                            );
                            let _ = socket.write_all(resp.as_bytes()).await;
                        }
                    });
                }
            }
        }
    });

    (addr, tx, request_count)
}

#[tokio::test]
async fn crawler_processes_all_fixture_pages_with_bounded_concurrency() {
    let total_urls = 100;
    let (addr, _shutdown, req_counter) = start_benchmark_server(total_urls).await;

    let start_url = format!("http://{}/item/1", addr);
    let session = Arc::new(Session::new());

    // Test Concurrency = 10
    let start_time = std::time::Instant::now();
    let mut crawler = Crawler::new(&start_url, session.clone())
        .limit(total_urls)
        .concurrency(10)
        .field(DatasetField {
            name: "heading".to_string(),
            selector: "h1".to_string(),
            selector_type: "css".to_string(),
            #[cfg(feature = "python")]
            transform: None,
            default: None,
            extract_type: crawlingo::extraction::ExtractionType::Text,
        });

    crawler.follow_selector = "a".into();
    crawler.respect_robots = false;
    let result = crawler.crawl_async().await.unwrap();
    let duration = start_time.elapsed();

    let pages_per_sec = (result.visited_count as f64) / duration.as_secs_f64();
    println!(
        "Phase 2 Crawler Benchmark (100 URLs, Concurrency=10): {:.2} ms | {:.2} pages/sec | Visited: {}",
        result.duration_ms, pages_per_sec, result.visited_count
    );

    assert_eq!(result.visited_count, total_urls);
    assert_eq!(req_counter.load(Ordering::Relaxed), total_urls);
    assert_eq!(result.failed_count, 0);
}
