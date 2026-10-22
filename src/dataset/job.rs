//! Bounded, resumable URL-file → dataset jobs. Output shards are committed before
//! their checkpoint, so an interrupted batch is replayed without publishing it twice.
use crate::engine::{fetcher::FetchRequest, session::Session};
use crate::error::{CrawlingoError, Result};
use crate::extraction::{ExtractionEngine, ExtractionType};
use crate::parser::{document::Page, streaming::HtmlParser};
use crate::selector::SelectorQuery;
use futures::{stream, StreamExt};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio_util::sync::CancellationToken;
use xxhash_rust::xxh64::Xxh64;

fn error(message: impl Into<String>) -> CrawlingoError {
    CrawlingoError::DatasetError(message.into())
}
fn json<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    serde_json::to_vec(value).map_err(|e| error(e.to_string()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct JobField {
    pub name: String,
    pub selector: String,
    pub selector_type: String,
    pub extract_type: String,
    pub required: bool,
    pub default: Option<String>,
    /// string, integer, float or boolean. Numeric coercion is strict.
    pub data_type: String,
    /// Collapse whitespace; disable for code/preformatted content.
    pub normalize_whitespace: bool,
    pub min_length: usize,
}
impl Default for JobField {
    fn default() -> Self {
        Self {
            name: String::new(),
            selector: String::new(),
            selector_type: "css".into(),
            extract_type: "text".into(),
            required: false,
            default: None,
            data_type: "string".into(),
            normalize_whitespace: true,
            min_length: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct JobConfig {
    /// UTF-8 text file: one HTTP(S) URL per line. Read lazily, never collected.
    pub urls_file: PathBuf,
    /// Exclusive directory owned by this job; reusing it resumes the same input/config.
    pub output_dir: PathBuf,
    pub fields: Vec<JobField>,
    /// CSS container selector for multiple independent records per page.
    pub row_selector: Option<String>,
    pub concurrency: usize,
    pub checkpoint_pages: usize,
    pub max_response_bytes: usize,
    pub max_records_per_page: usize,
    pub max_record_bytes: usize,
    /// Upper bound for the in-memory exact-dedup keys of one checkpoint batch.
    pub checkpoint_bytes: usize,
    pub deduplicate: bool,
    /// Empty means all extracted fields. Metadata such as source URL is excluded.
    pub dedup_fields: Vec<String>,
    pub respect_robots: bool,
    /// jsonl or csv. Each shard has a fixed schema and its own header for CSV.
    pub format: String,
}
impl Default for JobConfig {
    fn default() -> Self {
        Self {
            urls_file: PathBuf::new(),
            output_dir: PathBuf::new(),
            fields: Vec::new(),
            row_selector: None,
            concurrency: 16,
            checkpoint_pages: 256,
            max_response_bytes: 2 * 1024 * 1024,
            max_records_per_page: 1000,
            max_record_bytes: 256 * 1024,
            checkpoint_bytes: 32 * 1024 * 1024,
            deduplicate: true,
            dedup_fields: Vec::new(),
            respect_robots: true,
            format: "jsonl".into(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JobReport {
    pub source_lines: u64,
    pub pages_fetched: u64,
    pub pages_failed: u64,
    pub duplicate_urls: u64,
    pub records_written: u64,
    pub records_rejected: u64,
    pub duplicate_records: u64,
    pub shards: u64,
    pub complete: bool,
    pub cancelled: bool,
    /// Elapsed time for this invocation, including restart validation.
    pub elapsed_ms: u64,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct Checkpoint {
    offset: u64,
    report: JobReport,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct FileIntegrity {
    bytes: u64,
    sha256: String,
}

fn file_integrity(path: &Path) -> Result<FileIntegrity> {
    let mut file = File::open(path)?;
    let mut digest = Sha256::new();
    let mut buffer = [0u8; 65536];
    let mut bytes = 0u64;
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        digest.update(&buffer[..n]);
        bytes += n as u64;
    }
    Ok(FileIntegrity {
        bytes,
        sha256: format!("{:x}", digest.finalize()),
    })
}

fn sync_directory(path: &Path) -> Result<()> {
    #[cfg(unix)]
    File::open(path)?.sync_all()?;
    #[cfg(not(unix))]
    let _ = path;
    Ok(())
}

fn publish_json(path: &Path, value: &impl Serialize) -> Result<()> {
    let temporary = path.with_extension("json.partial");
    let mut file = File::create(&temporary)?;
    serde_json::to_writer(&mut file, value).map_err(|e| error(e.to_string()))?;
    file.sync_all()?;
    drop(file);
    std::fs::rename(temporary, path)?;
    sync_directory(path.parent().unwrap())
}

#[derive(Clone)]
pub struct DatasetJob {
    pub config: JobConfig,
    pub session: Arc<Session>,
    pub cancellation: CancellationToken,
}
impl DatasetJob {
    pub fn new(config: JobConfig, session: Arc<Session>) -> Result<Self> {
        validate_config(&config)?;
        Ok(Self {
            config,
            session,
            cancellation: CancellationToken::new(),
        })
    }
    pub fn cancel(&self) {
        self.cancellation.cancel();
    }
    pub async fn run(&self) -> Result<JobReport> {
        // Filesystem/Sled work never blocks Tokio's networking workers.
        let job = self.clone();
        let runtime = tokio::runtime::Handle::current();
        tokio::task::spawn_blocking(move || runtime.block_on(job.run_inner()))
            .await
            .map_err(|e| error(format!("dataset job worker failed: {e}")))?
    }
    async fn run_inner(&self) -> Result<JobReport> {
        let start = Instant::now();
        // One immutable configuration per run, shared by extraction workers.
        let shared_config = Arc::new(self.config.clone());
        let cfg = shared_config.as_ref();
        std::fs::create_dir_all(&cfg.output_dir)?;
        // Sled's exclusive lock prevents concurrent writers to this job directory.
        let db = sled::Config::new()
            .path(cfg.output_dir.join("state"))
            .cache_capacity(32 * 1024 * 1024)
            .open()?;
        let mut source = BufReader::new(File::open(&cfg.urls_file)?);
        let input = file_integrity(&cfg.urls_file)?;
        // Storage v2 binds the input to SHA-256 and requires committed shard integrity data.
        let signature = json(&(2u32, env!("CARGO_PKG_VERSION"), cfg, &input))?;
        match db.get(b"config")? {
            Some(previous) if previous.as_ref() != signature.as_slice() => {
                return Err(error(
                    "Input, configuration, engine version or checkpoint format changed; use a new output directory",
                ))
            }
            None => {
                db.insert(b"config", signature)?;
                db.flush()?;
            }
            _ => {}
        }
        let mut checkpoint: Checkpoint = match db.get(b"checkpoint")? {
            Some(bytes) => serde_json::from_slice(&bytes)
                .map_err(|e| error(format!("invalid checkpoint: {e}")))?,
            None => Checkpoint::default(),
        };
        // Check committed files before allowing progress. Orphan files beyond the checkpoint
        // are overwritten by replay; consumers must use report.shards as the commit boundary.
        for index in 0..checkpoint.report.shards {
            for suffix in [&cfg.format, "errors.jsonl"] {
                let name = format!("part-{index:08}.{suffix}");
                let path = cfg.output_dir.join(&name);
                if !path.is_file() {
                    return Err(error(format!(
                        "Missing committed output shard {index}; restore the job directory"
                    )));
                }
                let stored = db
                    .get(format!("file:{name}"))?
                    .ok_or_else(|| error(format!("Missing integrity metadata for {name}")))?;
                let expected: FileIntegrity = serde_json::from_slice(&stored)
                    .map_err(|e| error(format!("Invalid shard integrity metadata: {e}")))?;
                if file_integrity(&path)? != expected {
                    return Err(error(format!(
                        "Committed output shard {name} is corrupt; restore the job directory"
                    )));
                }
            }
        }
        source.seek(SeekFrom::Start(checkpoint.offset))?;
        let robots = Arc::new(crate::crawl::robots::RobotsCache::new());
        checkpoint.report.cancelled = false;
        loop {
            if self.cancellation.is_cancelled() {
                checkpoint.report.cancelled = true;
                break;
            }
            let mut inputs = Vec::with_capacity(cfg.checkpoint_pages);
            let mut offset = checkpoint.offset;
            for _ in 0..cfg.checkpoint_pages {
                // Bound even malformed URL lines before allocating them.
                let mut line = String::new();
                let n = source.by_ref().take(8194).read_line(&mut line)?;
                if n == 0 {
                    break;
                }
                if n > 8192 {
                    return Err(error("URL-file line exceeds 8192 bytes"));
                }
                offset += n as u64;
                inputs.push(line.trim().to_string());
            }
            if inputs.is_empty() {
                checkpoint.report.complete = true;
                db.insert(b"checkpoint", json(&checkpoint)?)?;
                db.flush()?;
                break;
            }
            let shard = checkpoint.report.shards;
            let data_path = cfg
                .output_dir
                .join(format!("part-{shard:08}.{}", cfg.format));
            let errors_path = cfg.output_dir.join(format!("part-{shard:08}.errors.jsonl"));
            let data_tmp = data_path.with_extension(format!("{}.partial", cfg.format));
            let errors_tmp = errors_path.with_extension("jsonl.partial");
            let mut output = ShardWriter::new(&data_tmp, cfg)?;
            let mut errors = BufWriter::new(File::create(&errors_tmp)?);
            let mut report = checkpoint.report.clone();
            let mut changes = sled::Batch::default();
            let mut pending_urls = HashSet::new();
            // Hash keys include the full canonical record as their value. Collision slots
            // preserve exactness instead of treating a hash collision as a duplicate.
            let mut pending_records: HashMap<Vec<u8>, sled::IVec> = HashMap::new();
            let mut pending_bytes = 0usize;
            let mut tasks = Vec::with_capacity(inputs.len());
            for raw in inputs {
                report.source_lines += 1;
                let url = match crate::util::url::UrlValidator::default().validate(&raw) {
                    Ok(url) => url.to_string(),
                    Err(e) => {
                        report.pages_failed += 1;
                        write_issue(&mut errors, &raw, None, &e.to_string())?;
                        continue;
                    }
                };
                let canonical =
                    crate::util::url::canonicalize_url(&url).unwrap_or_else(|| url.clone());
                let key = format!("url:{canonical}").into_bytes();
                if db.contains_key(&key)? || !pending_urls.insert(key.clone()) {
                    report.duplicate_urls += 1;
                    continue;
                }
                changes.insert(key, Vec::new());
                tasks.push(url);
            }
            let mut pages = stream::iter(tasks.into_iter().map(|url| {
                let robots = robots.clone();
                let config = shared_config.clone();
                async move {
                    let result = self.fetch_records(&url, &robots, config).await;
                    (url, result)
                }
            }))
            .buffered(cfg.concurrency);
            let mut interrupted = false;
            loop {
                let next = tokio::select! {
                    _ = self.cancellation.cancelled() => { interrupted = true; None },
                    next = pages.next() => next,
                };
                let Some((url, result)) = next else {
                    break;
                };
                match result {
                    Err(e) => {
                        report.pages_failed += 1;
                        write_issue(&mut errors, &url, None, &e.to_string())?;
                    }
                    Ok(records) => {
                        report.pages_fetched += 1;
                        for (row, record) in records.into_iter().enumerate() {
                            match record {
                                Err(e) => {
                                    report.records_rejected += 1;
                                    write_issue(&mut errors, &url, Some(row), &e.to_string())?;
                                }
                                Ok(mut fields) => {
                                    if cfg.deduplicate {
                                        let canonical = if cfg.dedup_fields.is_empty() {
                                            json(&fields)?
                                        } else {
                                            json(
                                                &cfg.dedup_fields
                                                    .iter()
                                                    .map(|key| (key, fields.get(key)))
                                                    .collect::<BTreeMap<_, _>>(),
                                            )?
                                        };
                                        let mut digest = Xxh64::new(0);
                                        digest.update(&canonical);
                                        let mut slot = 0u64;
                                        let key = loop {
                                            let key =
                                                format!("record:{:016x}:{slot}", digest.digest())
                                                    .into_bytes();
                                            let stored = match pending_records.get(&key) {
                                                Some(value) => Some(value.clone()),
                                                None => db.get(&key)?,
                                            };
                                            match stored {
                                                Some(value)
                                                    if value.as_ref() == canonical.as_slice() =>
                                                {
                                                    break None
                                                }
                                                Some(_) => slot += 1,
                                                None => break Some(key),
                                            }
                                        };
                                        let Some(key) = key else {
                                            report.duplicate_records += 1;
                                            continue;
                                        };
                                        pending_bytes += canonical.len() + key.len();
                                        if pending_bytes > cfg.checkpoint_bytes {
                                            return Err(error("checkpoint dedup memory limit reached; use a new job directory with fewer checkpoint_pages or a larger checkpoint_bytes limit"));
                                        }
                                        // Sled's IVec shares large payloads between the batch and
                                        // collision lookup instead of copying every record twice.
                                        let canonical = sled::IVec::from(canonical);
                                        changes.insert(key.clone(), canonical.clone());
                                        pending_records.insert(key, canonical);
                                    }
                                    fields.insert("_source_url".into(), url.clone());
                                    fields.insert("_row".into(), row.to_string());
                                    output.write(&fields)?;
                                    report.records_written += 1;
                                }
                            }
                        }
                    }
                }
            }
            if interrupted {
                checkpoint.report.cancelled = true;
                break;
            }
            output.finish()?;
            errors.flush()?;
            errors.get_ref().sync_all()?;
            drop(errors);
            replace_uncommitted(&data_tmp, &data_path)?;
            replace_uncommitted(&errors_tmp, &errors_path)?;
            sync_directory(&cfg.output_dir)?;
            for path in [&data_path, &errors_path] {
                let name = path.file_name().unwrap().to_string_lossy();
                changes.insert(
                    format!("file:{name}").as_bytes(),
                    json(&file_integrity(path)?)?,
                );
            }
            report.shards += 1;
            checkpoint = Checkpoint { offset, report };
            changes.insert(b"checkpoint", json(&checkpoint)?);
            // Queue progress, URL and record dedup indices commit together.
            db.apply_batch(changes)?;
            db.flush_async().await?;
        }
        checkpoint.report.elapsed_ms = start.elapsed().as_millis() as u64;
        let report_path = cfg.output_dir.join("report.json");
        // The Sled checkpoint is authoritative. These snapshots may lag after a hard kill;
        // reopening the job verifies shards and regenerates them before consumers read them.
        publish_json(&report_path, &checkpoint.report)?;
        let manifest_path = cfg.output_dir.join("manifest.jsonl");
        let temporary = cfg.output_dir.join("manifest.jsonl.partial");
        let mut manifest = BufWriter::new(File::create(&temporary)?);
        serde_json::to_writer(&mut manifest, &serde_json::json!({
            "kind": "dataset", "storage_version": 2, "engine_version": env!("CARGO_PKG_VERSION"),
            "input": input, "format": cfg.format, "fields": cfg.fields, "report": checkpoint.report,
        })).map_err(|e| error(e.to_string()))?;
        manifest.write_all(b"\n")?;
        for item in db.scan_prefix(b"file:") {
            let (key, value) = item?;
            let integrity: FileIntegrity =
                serde_json::from_slice(&value).map_err(|e| error(e.to_string()))?;
            serde_json::to_writer(&mut manifest, &serde_json::json!({
                "kind": "file", "name": String::from_utf8_lossy(&key[5..]), "bytes": integrity.bytes,
                "sha256": integrity.sha256,
            })).map_err(|e| error(e.to_string()))?;
            manifest.write_all(b"\n")?;
        }
        manifest.flush()?;
        manifest.get_ref().sync_all()?;
        drop(manifest);
        std::fs::rename(temporary, manifest_path)?;
        sync_directory(&cfg.output_dir)?;
        Ok(checkpoint.report)
    }

    async fn fetch_records(
        &self,
        url: &str,
        robots: &crate::crawl::robots::RobotsCache,
        cfg: Arc<JobConfig>,
    ) -> Result<Vec<Result<BTreeMap<String, String>>>> {
        let mut headers = self.session.read_headers();
        let agent = headers
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case("user-agent"))
            .map(|(_, value)| value.clone())
            .unwrap_or_else(|| "CrawlingoBot".into());
        if !headers
            .keys()
            .any(|key| key.eq_ignore_ascii_case("user-agent"))
        {
            headers.insert("User-Agent".into(), agent.clone());
        }
        if cfg.respect_robots {
            let rules = robots.get_or_fetch(&self.session, url).await;
            let parsed = url::Url::parse(url).map_err(|e| error(e.to_string()))?;
            let path = &parsed[url::Position::BeforePath..url::Position::AfterQuery];
            if !rules.is_allowed(&agent, path) {
                return Err(error("disallowed by robots.txt"));
            }
            robots.wait_delay(url, &rules, &agent).await;
        }
        let request = FetchRequest {
            url: url.to_string(),
            tier: *self.session.fetcher_tier.read().unwrap(),
            browser_profile: self.session.browser_profile.read().unwrap().clone(),
            headers,
            cookies: self.session.read_cookies(),
            proxy: self.session.get_next_proxy_with_auth(),
            timeout: Duration::from_secs(*self.session.timeout_seconds.read().unwrap()),
            retries: self.session.retries.read().unwrap().unwrap_or(3) as usize,
        };
        let response = self.session.fetch_manager().dispatch(request).await?;
        if !(200..300).contains(&response.status) {
            return Err(error(format!("HTTP {}", response.status)));
        }
        if response.body.len() > cfg.max_response_bytes {
            return Err(error("response exceeds job byte limit"));
        }
        tokio::task::spawn_blocking(move || {
            let page = HtmlParser::parse(response)?;
            extract_records(&page, &cfg)
        })
        .await
        .map_err(|e| error(format!("extraction worker failed: {e}")))?
    }
}

fn validate_config(cfg: &JobConfig) -> Result<()> {
    if cfg.urls_file.as_os_str().is_empty() || cfg.output_dir.as_os_str().is_empty() {
        return Err(error("urls_file and output_dir are required"));
    }
    if !(1..=1024).contains(&cfg.concurrency) || !(1..=10000).contains(&cfg.checkpoint_pages) {
        return Err(error(
            "concurrency must be 1..1024 and checkpoint_pages 1..10000",
        ));
    }
    if cfg.max_response_bytes == 0
        || cfg.max_records_per_page == 0
        || cfg.max_record_bytes == 0
        || cfg.checkpoint_bytes == 0
    {
        return Err(error("resource limits must be positive"));
    }
    if !["jsonl", "csv"].contains(&cfg.format.as_str()) {
        return Err(error("job format must be jsonl or csv"));
    }
    if cfg.fields.is_empty() {
        return Err(error("at least one extraction field is required"));
    }
    let mut names = HashSet::new();
    for field in &cfg.fields {
        if field.name.is_empty() || field.name.starts_with('_') || !names.insert(field.name.clone())
        {
            return Err(error(
                "field names must be unique, nonempty, and must not start with '_'",
            ));
        }
        if field.selector.is_empty() {
            return Err(error("field selector cannot be empty"));
        }
        if !["css", "xpath", "regex", "text", "after_text", "before_text"]
            .contains(&field.selector_type.as_str())
        {
            return Err(error("unknown selector type"));
        }
        if !["text", "price", "datetime", "url"].contains(&field.extract_type.as_str())
            && !field.extract_type.starts_with("attr:")
        {
            return Err(error("unknown extraction type"));
        }
        if !["string", "integer", "float", "boolean"].contains(&field.data_type.as_str()) {
            return Err(error("unknown field data_type"));
        }
        if field.selector_type == "regex" {
            regex::Regex::new(&field.selector).map_err(|e| error(e.to_string()))?;
        }
    }
    if cfg.dedup_fields.iter().any(|key| !names.contains(key)) {
        return Err(error("dedup_fields must refer to declared fields"));
    }
    Ok(())
}

#[cfg(feature = "python")]
#[pyo3::pyclass(name = "DatasetJob")]
pub struct PyDatasetJob {
    inner: DatasetJob,
}

#[cfg(feature = "python")]
#[pyo3::pymethods]
impl PyDatasetJob {
    #[new]
    fn new(config_json: &str, session: &crate::engine::session::PySession) -> pyo3::PyResult<Self> {
        let config = serde_json::from_str(config_json)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        Ok(Self {
            inner: DatasetJob::new(config, session.inner.clone())?,
        })
    }
    fn run(&self, py: pyo3::Python<'_>) -> pyo3::PyResult<String> {
        let job = self.inner.clone();
        let report = py.detach(move || crate::TOKIO_RUNTIME.block_on(job.run()))?;
        serde_json::to_string(&report)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }
    fn cancel(&self) {
        self.inner.cancel();
    }
}

pub fn extract_records(
    page: &Page,
    cfg: &JobConfig,
) -> Result<Vec<Result<BTreeMap<String, String>>>> {
    let roots = match &cfg.row_selector {
        Some(selector) => page.query(SelectorQuery::Css(selector))?,
        None => vec![usize::MAX],
    };
    if roots.is_empty() {
        return Err(error("row selector matched no records"));
    }
    if roots.len() > cfg.max_records_per_page {
        return Err(error("page exceeds max_records_per_page"));
    }
    let tree = page.dom_tree();
    let root_rows: HashMap<_, _> = roots
        .iter()
        .enumerate()
        .map(|(row, index)| (*index, row))
        .collect();
    let mut owners = vec![None; tree.nodes.len()];
    for node in &tree.nodes {
        owners[node.index] = if cfg.row_selector.is_none() {
            Some(0)
        } else {
            root_rows
                .get(&node.index)
                .copied()
                .or_else(|| node.parent.and_then(|p| owners[p]))
        };
    }
    let mut rows = vec![BTreeMap::new(); roots.len()];
    for field in &cfg.fields {
        let matches = page.query(SelectorQuery::from_type_str(
            &field.selector_type,
            &field.selector,
        ))?;
        let mut grouped = vec![Vec::new(); roots.len()];
        for index in matches {
            if let Some(row) = owners[index] {
                grouped[row].push(index);
            }
        }
        let extract_type = ExtractionType::from_str_or_text(&field.extract_type);
        for (row, indices) in grouped.iter().enumerate() {
            let mut value = match &extract_type {
                ExtractionType::Attribute(attribute) => page.get_node_attr(indices, attribute),
                other => ExtractionEngine::normalize_value(
                    &page.get_nodes_combined_text(indices),
                    other,
                    page.url(),
                ),
            };
            if value.is_empty() {
                value = field.default.clone().unwrap_or_default();
            }
            if field.normalize_whitespace {
                value = value.split_whitespace().collect::<Vec<_>>().join(" ");
            }
            rows[row].insert(field.name.clone(), value);
        }
    }
    Ok(rows
        .into_iter()
        .map(|mut fields| {
            for field in &cfg.fields {
                let value = fields.get_mut(&field.name).unwrap();
                if field.required && value.trim().is_empty() {
                    return Err(error(format!("missing required field '{}'", field.name)));
                }
                if field.min_length > 0
                    && value.chars().take(field.min_length).count() < field.min_length
                {
                    return Err(error(format!("field '{}' below min_length", field.name)));
                }
                if !value.is_empty() {
                    let invalid = || {
                        error(format!(
                            "field '{}' is not a valid {}",
                            field.name, field.data_type
                        ))
                    };
                    let normalized = match field.data_type.as_str() {
                        "integer" => value
                            .trim()
                            .parse::<i64>()
                            .map_err(|_| invalid())?
                            .to_string(),
                        "float" => {
                            let number = value.trim().parse::<f64>().map_err(|_| invalid())?;
                            if !number.is_finite() {
                                return Err(invalid());
                            }
                            number.to_string()
                        }
                        "boolean" => match value.to_lowercase().as_str() {
                            "true" | "yes" | "1" => "true".into(),
                            "false" | "no" | "0" => "false".into(),
                            _ => return Err(invalid()),
                        },
                        _ => continue,
                    };
                    *value = normalized;
                }
            }
            if fields.values().all(|value| value.trim().is_empty()) {
                return Err(error("empty record"));
            }
            if json(&fields)?.len() > cfg.max_record_bytes {
                return Err(error("record exceeds max_record_bytes"));
            }
            Ok(fields)
        })
        .collect())
}

fn write_issue(
    writer: &mut impl Write,
    url: &str,
    row: Option<usize>,
    message: &str,
) -> Result<()> {
    serde_json::to_writer(
        &mut *writer,
        &serde_json::json!({"url":url,"row":row,"error":message}),
    )
    .map_err(|e| error(e.to_string()))?;
    writer.write_all(b"\n")?;
    Ok(())
}
fn replace_uncommitted(from: &Path, to: &Path) -> Result<()> {
    // Only called for the next uncommitted shard inside this job's exclusive directory.
    if to.exists() {
        std::fs::remove_file(to)?;
    }
    std::fs::rename(from, to)?;
    Ok(())
}
enum ShardWriter {
    Json(BufWriter<File>),
    Csv(Box<csv::Writer<File>>, Vec<String>),
}
impl ShardWriter {
    fn new(path: &Path, cfg: &JobConfig) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)?;
        if cfg.format == "csv" {
            let keys: Vec<String> = cfg
                .fields
                .iter()
                .map(|f| f.name.clone())
                .chain(["_source_url".into(), "_row".into()])
                .collect();
            let mut writer = csv::Writer::from_writer(file);
            writer.write_record(&keys)?;
            Ok(Self::Csv(Box::new(writer), keys))
        } else {
            Ok(Self::Json(BufWriter::new(file)))
        }
    }
    fn write(&mut self, fields: &BTreeMap<String, String>) -> Result<()> {
        match self {
            Self::Json(writer) => {
                serde_json::to_writer(&mut *writer, fields).map_err(|e| error(e.to_string()))?;
                writer.write_all(b"\n")?;
            }
            Self::Csv(writer, keys) => writer.write_record(
                keys.iter()
                    .map(|key| fields.get(key).map(String::as_str).unwrap_or("")),
            )?,
        }
        Ok(())
    }
    fn finish(self) -> Result<()> {
        match self {
            Self::Json(mut writer) => {
                writer.flush()?;
                writer.get_ref().sync_all()?;
            }
            Self::Csv(mut writer, _) => {
                writer.flush()?;
                writer.get_ref().sync_all()?;
            }
        }
        Ok(())
    }
}
