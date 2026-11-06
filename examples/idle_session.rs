//! Memory probe: one idle Session plus a four-worker Tokio runtime, no HTTP server.
//! Measure the resident process externally; binary size is not runtime memory.
fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();
    let session = crawlingo::engine::session::Session::new();
    println!(
        "{}",
        serde_json::json!({"state":"idle", "runtime_workers":4, "http_clients":0, "scope":"Rust Session and Tokio runtime; no browser, SDK, fixture server or dataset"})
    );
    std::thread::sleep(std::time::Duration::from_secs(5));
    std::hint::black_box(&session);
    drop(runtime);
}
