//! Import an immutable Tranco top-million snapshot using Crawlingo's HTTP transport.
//! cargo run --release --example tranco_directory -- JZPGY target/tranco-JZPGY
use crawlingo::engine::fetcher::{FetchRequest, FetcherTier, HttpFetcher, Transport};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use std::time::{Duration, Instant};

const ROWS: usize = 1_000_000;

#[derive(Serialize, Deserialize)]
struct RankedDomain {
    rank: usize,
    domain: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let started = Instant::now();
    let mut args = std::env::args().skip(1);
    let id = args.next().ok_or("provide a Tranco list ID")?;
    let dir = PathBuf::from(args.next().ok_or("provide a new output directory")?);
    if args.next().is_some() || id.is_empty() || !id.bytes().all(|c| c.is_ascii_alphanumeric()) {
        return Err("expected a Tranco list ID and a new output directory".into());
    }
    fs::create_dir(&dir)?;
    let url = format!("https://tranco-list.eu/download/{id}/{ROWS}");
    let transport = HttpFetcher::new(Default::default()).with_max_response_bytes(64 << 20);
    let response = transport
        .fetch(&FetchRequest {
            url: url.clone(),
            tier: FetcherTier::Standard,
            browser_profile: None,
            headers: HashMap::from([(
                "User-Agent".into(),
                "CrawlingoDatasetEvaluation/1.0 (+https://github.com/Vamshavardhan50/crawlingo)"
                    .into(),
            )]),
            cookies: Default::default(),
            proxy: None,
            timeout: Duration::from_secs(120),
            retries: 0,
        })
        .await?;
    let fetch_seconds = started.elapsed().as_secs_f64();
    if response.status != 200 {
        return Err(format!("Tranco returned HTTP {}", response.status).into());
    }
    let csv_reader = || {
        csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(response.body.as_ref())
    };
    let partial = dir.join("domains.jsonl.partial");
    let mut domains = HashSet::with_capacity(ROWS);
    let mut output = BufWriter::new(File::create(&partial)?);
    for (index, row) in csv_reader().records().enumerate() {
        let row = row?;
        let rank = index + 1;
        let domain = row.get(1).ok_or("missing domain")?;
        if rank > ROWS
            || row.len() != 2
            || row[0].parse::<usize>()? != rank
            || domain.is_empty()
            || domain.contains(char::is_whitespace)
            || !domains.insert(domain.to_ascii_lowercase())
        {
            return Err(format!("invalid rank, domain or duplicate at CSV record {rank}").into());
        }
        serde_json::to_writer(
            &mut output,
            &RankedDomain {
                rank,
                domain: domain.to_owned(),
            },
        )?;
        output.write_all(b"\n")?;
    }
    if domains.len() != ROWS {
        return Err(format!("expected {ROWS} unique domains, got {}", domains.len()).into());
    }
    drop(domains);
    output.flush()?;
    output.get_ref().sync_all()?;
    drop(output);
    let export_seconds = started.elapsed().as_secs_f64();

    let mut input = BufReader::new(File::open(&partial)?);
    let mut digest = Sha256::new();
    let mut line = Vec::new();
    for row in csv_reader().records() {
        let row = row?;
        line.clear();
        input.read_until(b'\n', &mut line)?;
        let record: RankedDomain = serde_json::from_slice(&line)?;
        if record.rank != row[0].parse::<usize>()? || record.domain != row[1] {
            return Err("export differs from the downloaded source".into());
        }
        digest.update(&line);
    }
    if !input.fill_buf()?.is_empty() {
        return Err("output contains extra records".into());
    }
    drop(input);
    fs::rename(partial, dir.join("domains.jsonl"))?;
    let report = serde_json::json!({
        "source":url, "snapshot_id":id, "captured_at_utc":chrono::Utc::now(),
        "http_status":response.status, "source_bytes":response.body.len(),
        "source_sha256":format!("{:x}", Sha256::digest(&response.body)),
        "server_cache_status":response.headers.iter().find(|(k,_)| k.eq_ignore_ascii_case("cf-cache-status")).map(|(_,v)| v),
        "verified_unique_records":ROWS, "jsonl_sha256":format!("{:x}", digest.finalize()),
        "fetch_seconds":fetch_seconds, "export_seconds_from_start":export_seconds,
        "verified_seconds_from_start":started.elapsed().as_secs_f64(),
        "scope":"One HTTPS snapshot download using Crawlingo, CSV validation and uniqueness check, durable JSONL export and comparison of every output record with the response."
    });
    let evidence = serde_json::to_vec_pretty(&report)?;
    fs::write(dir.join("report.json"), &evidence)?;
    println!("{}", String::from_utf8(evidence)?);
    Ok(())
}
