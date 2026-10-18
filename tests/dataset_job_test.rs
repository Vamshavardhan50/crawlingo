use crawlingo::dataset::{DatasetJob, JobConfig, JobField};
use crawlingo::engine::fetcher::{
    BoxFuture, FetchRequest, MockTransport, NormalizedResponse, ResponseTimings, Transport,
};
use crawlingo::engine::session::Session;
use crawlingo::error::Result;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::sync::Arc;

fn config(dir: &std::path::Path, urls: &str) -> JobConfig {
    let file = dir.join("urls.txt");
    std::fs::write(&file, urls).unwrap();
    JobConfig {
        urls_file: file,
        output_dir: dir.join("job"),
        fields: vec![JobField {
            name: "title".into(),
            selector: "h1".into(),
            required: true,
            ..Default::default()
        }],
        respect_robots: false,
        checkpoint_pages: 2,
        concurrency: 2,
        ..Default::default()
    }
}
fn records(cfg: &JobConfig, shards: u64) -> Vec<serde_json::Value> {
    (0..shards)
        .flat_map(|i| {
            BufReader::new(
                std::fs::File::open(cfg.output_dir.join(format!("part-{i:08}.jsonl"))).unwrap(),
            )
            .lines()
            .map(|l| serde_json::from_str(&l.unwrap()).unwrap())
            .collect::<Vec<_>>()
        })
        .collect()
}

#[tokio::test(flavor = "multi_thread")]
async fn robots_uses_configured_agent_and_query_string() {
    let dir = tempfile::tempdir().unwrap();
    let mut cfg = config(
        dir.path(),
        "https://fixture.test/a?private=1\nhttps://fixture.test/b\n",
    );
    cfg.respect_robots = true;
    let mock = Arc::new(
        MockTransport::new()
            .with_html(
                "https://fixture.test/robots.txt",
                "User-agent: DatasetTest\nDisallow: /*?private=1$\nUser-agent: *\nDisallow: /",
            )
            .with_html("https://fixture.test/a?private=1", "<h1>Private</h1>")
            .with_html("https://fixture.test/b", "<h1>Public</h1>"),
    );
    let session = Arc::new(Session::new());
    session
        .headers
        .write()
        .unwrap()
        .insert("User-Agent".into(), "DatasetTest".into());
    session.set_transport(mock);
    let report = DatasetJob::new(cfg.clone(), session)
        .unwrap()
        .run()
        .await
        .unwrap();
    assert_eq!(report.pages_failed, 1);
    assert_eq!(report.pages_fetched, 1);
    assert_eq!(records(&cfg, report.shards)[0]["title"], "Public");
}

#[tokio::test(flavor = "multi_thread")]
async fn csv_preserves_declared_column_order_and_escaping() {
    let dir = tempfile::tempdir().unwrap();
    let mut cfg = config(dir.path(), "https://fixture.test/a\n");
    cfg.format = "csv".into();
    cfg.fields.push(JobField {
        name: "id".into(),
        selector: "h1".into(),
        extract_type: "attr:data-id".into(),
        ..Default::default()
    });
    let session = Arc::new(Session::new());
    session.set_transport(Arc::new(MockTransport::new().with_html(
        "https://fixture.test/a",
        "<h1 data-id='42'>café, title</h1>",
    )));
    let report = DatasetJob::new(cfg.clone(), session)
        .unwrap()
        .run()
        .await
        .unwrap();
    assert_eq!(report.records_written, 1);
    let mut csv = csv::Reader::from_path(cfg.output_dir.join("part-00000000.csv")).unwrap();
    assert_eq!(
        csv.headers().unwrap().iter().collect::<Vec<_>>(),
        vec!["title", "id", "_source_url", "_row"]
    );
    let row = csv.records().next().unwrap().unwrap();
    assert_eq!(&row[0], "café, title");
    assert_eq!(&row[1], "42");
}

#[tokio::test(flavor = "multi_thread")]
async fn rows_remain_aligned_and_rejections_are_reported() {
    let dir = tempfile::tempdir().unwrap();
    let mut cfg = config(
        dir.path(),
        "https://fixture.test/a\nhttps://fixture.test/b\nhttps://fixture.test/a\n",
    );
    cfg.row_selector = Some("article".into());
    cfg.fields.push(JobField {
        name: "price".into(),
        selector: ".price".into(),
        data_type: "integer".into(),
        default: Some("0".into()),
        ..Default::default()
    });
    cfg.fields.push(JobField {
        name: "id".into(),
        selector: "article".into(),
        extract_type: "attr:data-id".into(),
        ..Default::default()
    });
    let mock = MockTransport::new().with_html("https://fixture.test/a", "<article data-id='1'><h1> A  title </h1></article><article data-id='2'><h1>B</h1><b class='price'>9</b></article><article><b class='price'>bad</b></article>").with_html("https://fixture.test/b", "<article data-id='2'><h1>B</h1><b class='price'>9</b></article>");
    let session = Arc::new(Session::new());
    session.set_transport(Arc::new(mock));
    let report = DatasetJob::new(cfg.clone(), session)
        .unwrap()
        .run()
        .await
        .unwrap();
    assert_eq!(report.pages_fetched, 2);
    assert_eq!(report.records_written, 2);
    assert_eq!(report.records_rejected, 1);
    assert_eq!(report.duplicate_urls, 1);
    assert_eq!(report.duplicate_records, 1);
    let rows = records(&cfg, report.shards);
    assert_eq!(rows[0]["price"], "0");
    assert_eq!(rows[1]["price"], "9");
    assert_eq!(rows[0]["title"], "A title");
    assert_eq!(rows[0]["id"], "1");
    assert!(report.complete);
}

#[tokio::test(flavor = "multi_thread")]
async fn resume_is_idempotent_and_changed_input_is_rejected() {
    let dir = tempfile::tempdir().unwrap();
    let cfg = config(
        dir.path(),
        "https://fixture.test/a\nhttps://fixture.test/b\n",
    );
    let mock = Arc::new(
        MockTransport::new()
            .with_html("https://fixture.test/a", "<h1>A</h1>")
            .with_html("https://fixture.test/b", "<h1>B</h1>"),
    );
    let session = Arc::new(Session::new());
    session.set_transport(mock.clone());
    let job = DatasetJob::new(cfg.clone(), session).unwrap();
    let first = job.run().await.unwrap();
    let second = job.run().await.unwrap();
    assert_eq!(first.records_written, second.records_written);
    assert_eq!(mock.call_count(), 2);
    std::fs::write(&cfg.urls_file, "https://fixture.test/c\n").unwrap();
    assert!(job.run().await.is_err());
}

#[tokio::test(flavor = "multi_thread")]
async fn errors_are_not_exported_as_valid_rows_and_csv_keys_are_stable() {
    let dir = tempfile::tempdir().unwrap();
    let mut cfg = config(
        dir.path(),
        "https://fixture.test/a\nnot-a-url\nhttps://fixture.test/empty\n",
    );
    cfg.format = "csv".into();
    let session = Arc::new(Session::new());
    session.set_transport(Arc::new(
        MockTransport::new()
            .with_html("https://fixture.test/a", "<h1>Text, with comma</h1>")
            .with_html("https://fixture.test/empty", "<div></div>"),
    ));
    let report = DatasetJob::new(cfg.clone(), session)
        .unwrap()
        .run()
        .await
        .unwrap();
    assert_eq!(report.records_written, 1);
    assert_eq!(report.pages_failed, 1);
    assert_eq!(report.records_rejected, 1);
    let mut csv = csv::Reader::from_path(cfg.output_dir.join("part-00000000.csv")).unwrap();
    assert_eq!(csv.headers().unwrap().get(0), Some("title"));
    assert_eq!(
        csv.records().next().unwrap().unwrap().get(0),
        Some("Text, with comma")
    );
}

struct Delayed;
impl Transport for Delayed {
    fn fetch<'a>(&'a self, request: &'a FetchRequest) -> BoxFuture<'a, Result<NormalizedResponse>> {
        Box::pin(async move {
            if request.url.ends_with("/slow") {
                tokio::time::sleep(std::time::Duration::from_secs(30)).await;
            }
            Ok(NormalizedResponse {
                url: request.url.clone(),
                status: 200,
                headers: HashMap::new(),
                cookies: HashMap::new(),
                body: format!("<h1>{}</h1>", request.url).into(),
                content_type: "text/html".into(),
                encoding: "utf-8".into(),
                timings: ResponseTimings::default(),
            })
        })
    }
}
#[tokio::test(flavor = "multi_thread")]
async fn cancelled_batch_replays_without_losing_committed_rows() {
    let dir = tempfile::tempdir().unwrap();
    let mut cfg = config(
        dir.path(),
        "https://fixture.test/fast\nhttps://fixture.test/slow\n",
    );
    cfg.checkpoint_pages = 1;
    let session = Arc::new(Session::new());
    session.set_transport(Arc::new(Delayed));
    let job = DatasetJob::new(cfg.clone(), session).unwrap();
    let cancel = job.cancellation.clone();
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        cancel.cancel();
    });
    let report = job.run().await.unwrap();
    assert!(report.cancelled);
    assert!(!report.complete);
    let session = Arc::new(Session::new());
    session.set_transport(Arc::new(
        MockTransport::new()
            .with_html(
                "https://fixture.test/fast",
                "<h1>https://fixture.test/fast</h1>",
            )
            .with_html("https://fixture.test/slow", "<h1>Slow</h1>"),
    ));
    let resumed = DatasetJob::new(cfg.clone(), session)
        .unwrap()
        .run()
        .await
        .unwrap();
    assert_eq!(resumed.records_written, 2);
    assert_eq!(records(&cfg, resumed.shards).len(), 2);
}

#[tokio::test]
async fn parquet_writer_handles_more_than_channel_capacity() {
    use crawlingo::dataset::stream::DatasetStream;
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("many.parquet");
    let stream = DatasetStream::with_capacity(4);
    let producer = stream.handle();
    tokio::spawn(async move {
        for i in 0..501 {
            producer
                .send(Ok(HashMap::from([("id".into(), i.to_string())])))
                .await
                .unwrap();
        }
    });
    let count = tokio::time::timeout(
        std::time::Duration::from_secs(10),
        stream.write_parquet(path.to_str().unwrap()),
    )
    .await
    .unwrap()
    .unwrap();
    assert_eq!(count, 501);
    use parquet::file::reader::FileReader;
    let reader =
        parquet::file::reader::SerializedFileReader::new(std::fs::File::open(path).unwrap())
            .unwrap();
    assert_eq!(reader.metadata().file_metadata().num_rows(), 501);
}

#[tokio::test]
async fn stream_producer_observes_backpressure_and_receiver_drop() {
    let mut stream = crawlingo::dataset::stream::DatasetStream::with_capacity(1);
    let handle = stream.handle();
    handle.send(Ok(HashMap::new())).await.unwrap();
    assert!(tokio::time::timeout(
        std::time::Duration::from_millis(20),
        handle.send(Ok(HashMap::new()))
    )
    .await
    .is_err());
    stream.recv().await.unwrap().unwrap();
    drop(stream);
    assert!(handle.send(Ok(HashMap::new())).await.is_err());
}
