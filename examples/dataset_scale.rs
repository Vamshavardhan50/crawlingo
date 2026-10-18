//! Controlled scale acceptance: cargo run --release --example dataset_scale -- 1000000 target/scale-run http
//! Mode `generated` replaces HTTP only; both modes use the production dataset job.
use crawlingo::dataset::{DatasetJob, JobConfig, JobField};
use crawlingo::engine::fetcher::{
    BoxFuture, FetchRequest, NormalizedResponse, ResponseTimings, Transport,
};
use crawlingo::engine::session::Session;
use crawlingo::error::Result;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

fn html(id: &str) -> String {
    format!("<html><body><article><h1>Document {id}</h1><p class='content'>A controlled fixture for record {id}. Unicode: café λ 🚀.</p><span class='id'>{id}</span></article></body></html>")
}
struct Generated {
    calls: Arc<AtomicU64>,
}
impl Transport for Generated {
    fn fetch<'a>(&'a self, request: &'a FetchRequest) -> BoxFuture<'a, Result<NormalizedResponse>> {
        Box::pin(async move {
            self.calls.fetch_add(1, Ordering::Relaxed);
            let id = request.url.rsplit('/').next().unwrap();
            Ok(NormalizedResponse {
                url: request.url.clone(),
                status: 200,
                headers: HashMap::new(),
                cookies: HashMap::new(),
                body: html(id).into(),
                content_type: "text/html".into(),
                encoding: "utf-8".into(),
                timings: ResponseTimings::default(),
            })
        })
    }
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let count: u64 = args.get(1).map(String::as_str).unwrap_or("10000").parse()?;
    let dir = std::path::PathBuf::from(args.get(2).ok_or("provide a new output directory")?);
    let mode = args.get(3).map(String::as_str).unwrap_or("generated");
    if dir.exists() {
        return Err("scale acceptance requires a new output directory".into());
    }
    std::fs::create_dir_all(&dir)?;
    let calls = Arc::new(AtomicU64::new(0));
    let session = Arc::new(Session::new());
    *session.retries.write().unwrap() = Some(0);
    let mut server = None;
    let origin = if mode == "http" {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
        let address = listener.local_addr()?;
        let calls = calls.clone();
        server = Some(tokio::spawn(async move {
            while let Ok((socket, _)) = listener.accept().await {
                let calls = calls.clone();
                tokio::spawn(async move {
                    let (reader, mut writer) = socket.into_split();
                    let mut reader = tokio::io::BufReader::new(reader);
                    let mut line = String::new();
                    loop {
                        line.clear();
                        if reader.read_line(&mut line).await.unwrap_or(0) == 0 {
                            break;
                        }
                        let path = line.split_whitespace().nth(1).unwrap_or("/").to_string();
                        loop {
                            line.clear();
                            if reader.read_line(&mut line).await.unwrap_or(0) == 0 {
                                return;
                            }
                            if line == "\r\n" {
                                break;
                            }
                        }
                        let body = html(path.rsplit('/').next().unwrap_or("0"));
                        calls.fetch_add(1, Ordering::Relaxed);
                        let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: keep-alive\r\n\r\n{}", body.len(), body);
                        if writer.write_all(response.as_bytes()).await.is_err() {
                            break;
                        }
                    }
                });
            }
        }));
        format!("http://{address}")
    } else if mode == "generated" {
        session.set_transport(Arc::new(Generated {
            calls: calls.clone(),
        }));
        "https://fixture.test".into()
    } else {
        return Err("mode must be generated or http".into());
    };
    let urls_file = dir.join("urls.txt");
    let mut urls = BufWriter::new(std::fs::File::create(&urls_file)?);
    for id in 0..count {
        writeln!(urls, "{origin}/{id}")?;
    }
    urls.flush()?;
    drop(urls);
    let cfg = JobConfig {
        urls_file,
        output_dir: dir.join("dataset"),
        concurrency: 50,
        checkpoint_pages: 1000,
        respect_robots: false,
        row_selector: Some("article".into()),
        fields: vec![
            JobField {
                name: "id".into(),
                selector: ".id".into(),
                required: true,
                data_type: "integer".into(),
                ..Default::default()
            },
            JobField {
                name: "title".into(),
                selector: "h1".into(),
                required: true,
                ..Default::default()
            },
            JobField {
                name: "content".into(),
                selector: ".content".into(),
                required: true,
                ..Default::default()
            },
        ],
        ..Default::default()
    };
    let report = DatasetJob::new(cfg.clone(), session.clone())?.run().await?;
    assert!(report.complete);
    assert_eq!(report.pages_failed, 0);
    assert_eq!(report.pages_fetched, count);
    assert_eq!(report.records_written, count);
    assert_eq!(calls.load(Ordering::Relaxed), count);
    // Stream-verify every row, field and ordering without collecting one million objects.
    let mut verified = 0u64;
    for shard in 0..report.shards {
        let file = std::fs::File::open(cfg.output_dir.join(format!("part-{shard:08}.jsonl")))?;
        for line in BufReader::new(file).lines() {
            let record: serde_json::Value = serde_json::from_str(&line?)?;
            assert_eq!(record["id"], verified.to_string());
            assert_eq!(record["title"], format!("Document {verified}"));
            assert!(record["content"].as_str().unwrap().contains("café λ 🚀"));
            verified += 1;
        }
    }
    assert_eq!(verified, count);
    let resumed = DatasetJob::new(cfg, session)?.run().await?;
    assert_eq!(resumed.records_written, count);
    assert_eq!(calls.load(Ordering::Relaxed), count);
    if let Some(server) = server {
        server.abort();
    }
    let result = serde_json::json!({"mode": mode, "pages": count, "verified_records": verified, "concurrency":50, "checkpoint_pages":1000, "report":report, "resume_verified":true, "fixture":"small generated UTF-8 HTML; local HTTP server shares the process when enabled", "profile":if cfg!(debug_assertions){"debug"}else{"release"}, "os":std::env::consts::OS,"arch":std::env::consts::ARCH});
    std::fs::write(
        dir.join("acceptance.json"),
        serde_json::to_vec_pretty(&result)?,
    )?;
    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}
