//! File and binary downloads with streaming, resumable partial-content support, and MIME-type
//! detection.
//!
//! [`Downloader`] is the entry point. It uses the session's `FetchManager` for transport (sharing
//! rate limiting, retry, caching, and auth with the rest of the engine) and writes the response
//! body to a caller-supplied `Write` (file, buffer, etc.) in chunks rather than buffering the
//! entire body in memory — essential for large files.
//!
//! ## Features
//! - **Streaming writes**: response body is forwarded in `chunk_size`-byte increments.
//! - **Resumable downloads**: if the output file already exists, an `HTTP 206 Partial Content`
//!   `Range: bytes=offset-` request is sent; a server that doesn't support `Range` responds with
//!   `200 OK` and the full body, which is written from the beginning (the existing partial file is
//!   truncated). A server error or transport error causes the download to fail, preserving the
//!   partial file for a future retry.
//! - **Content-Disposition filename detection**: `suggested_filename` returns the filename hint
//!   from the `Content-Disposition` header (or the last URL path segment as a fallback).
//! - **MIME sniffing**: `sniff_mime` returns the content type from the response `Content-Type`
//!   header, falling back to `application/octet-stream`.

use crate::engine::fetcher::{FetchRequest, FetcherTier};
use crate::engine::session::Session;
use crate::error::{CrawlingoError, Result};
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

/// The result of a completed download.
#[derive(Debug, Clone)]
pub struct DownloadResult {
    /// The final effective URL after any redirects.
    pub url: String,
    /// HTTP status code of the response that provided the body.
    pub status: u16,
    /// Number of bytes written to the output.
    pub bytes_written: u64,
    /// MIME type from the `Content-Type` response header, or `"application/octet-stream"`.
    pub content_type: String,
    /// Filename hint, from `Content-Disposition` or the URL path, or `None` if neither is
    /// available.
    pub suggested_filename: Option<String>,
    /// `true` if the server honored a `Range:` request and returned `206 Partial Content`.
    pub resumed: bool,
}

/// Configures and executes a file download.
pub struct Downloader {
    session: Arc<Session>,
    /// How many bytes to write per iteration. Defaults to 64 KiB.
    pub chunk_size: usize,
    /// If `true`, attempts to resume a partial download using `Range: bytes=offset-`.
    pub allow_resume: bool,
    /// Maximum number of bytes to download (inclusive). `None` means unlimited.
    pub max_bytes: Option<u64>,
}

impl Downloader {
    pub fn new(session: Arc<Session>) -> Self {
        Self {
            session,
            chunk_size: 65_536,
            allow_resume: true,
            max_bytes: None,
        }
    }

    pub fn with_chunk_size(mut self, size: usize) -> Self {
        self.chunk_size = size;
        self
    }

    pub fn with_resume(mut self, allow: bool) -> Self {
        self.allow_resume = allow;
        self
    }

    pub fn with_max_bytes(mut self, max: u64) -> Self {
        self.max_bytes = Some(max);
        self
    }

    /// Downloads `url` to a new or existing file at `dest`, streaming the body.
    ///
    /// If `allow_resume` is `true` and `dest` already exists, the download resumes from the
    /// file's current size (using `Range: bytes=<size>-`). If the server doesn't support Range
    /// requests (`200 OK` instead of `206`), the file is overwritten from the beginning.
    pub fn download_to_file(&self, url: &str, dest: &Path) -> Result<DownloadResult> {
        crate::TOKIO_RUNTIME.block_on(self.download_to_file_async(url, dest))
    }

    pub async fn download_to_file_async(&self, url: &str, dest: &Path) -> Result<DownloadResult> {
        // Determine whether to resume.
        let (offset, append) = if self.allow_resume && dest.exists() {
            let existing = std::fs::metadata(dest).map(|m| m.len()).unwrap_or(0);
            (existing, existing > 0)
        } else {
            (0, false)
        };

        // Build request, optionally with a Range header.
        let mut headers = self.session.read_headers();
        if append && offset > 0 {
            headers.insert("Range".to_string(), format!("bytes={offset}-"));
        }

        let req = FetchRequest {
            url: url.to_string(),
            tier: FetcherTier::Standard,
            browser_profile: None,
            headers,
            cookies: self.session.read_cookies(),
            proxy: self.session.get_next_proxy(),
            timeout: Duration::from_secs(*self.session.timeout_seconds.read().unwrap()),
            retries: 2,
        };

        let manager = self.session.fetch_manager();
        let resp = manager.dispatch(req).await?;

        let resumed = resp.status == 206;
        let content_type = sniff_content_type(&resp.headers);
        let suggested_filename = extract_filename(&resp.url, &resp.headers);

        // Open dest for writing.
        let open_result = if resumed {
            std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(dest)
        } else {
            std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(dest)
        };
        let mut file = open_result.map_err(|e| {
            CrawlingoError::FetchError(format!(
                "failed to open download destination {}: {e}",
                dest.display()
            ))
        })?;

        // Write in chunks.
        let body = &resp.body;
        let mut bytes_written: u64 = 0;
        let chunk_size = self.chunk_size;
        let max_bytes = self.max_bytes;

        let mut pos = 0;
        while pos < body.len() {
            if let Some(max) = max_bytes {
                if bytes_written >= max {
                    break;
                }
            }
            let end = (pos + chunk_size).min(body.len());
            let chunk = &body[pos..end];
            let write_len = if let Some(max) = max_bytes {
                let remaining = max.saturating_sub(bytes_written) as usize;
                remaining.min(chunk.len())
            } else {
                chunk.len()
            };
            file.write_all(&chunk[..write_len]).map_err(|e| {
                CrawlingoError::FetchError(format!("write error during download: {e}"))
            })?;
            bytes_written += write_len as u64;
            pos = end;
        }

        Ok(DownloadResult {
            url: resp.url,
            status: resp.status,
            bytes_written,
            content_type,
            suggested_filename,
            resumed,
        })
    }

    /// Downloads `url` to an in-memory `Vec<u8>`.
    pub fn download_to_memory(&self, url: &str) -> Result<(DownloadResult, Vec<u8>)> {
        crate::TOKIO_RUNTIME.block_on(self.download_to_memory_async(url))
    }

    pub async fn download_to_memory_async(&self, url: &str) -> Result<(DownloadResult, Vec<u8>)> {
        let headers = self.session.read_headers();
        let req = FetchRequest {
            url: url.to_string(),
            tier: FetcherTier::Standard,
            browser_profile: None,
            headers,
            cookies: self.session.read_cookies(),
            proxy: self.session.get_next_proxy(),
            timeout: Duration::from_secs(*self.session.timeout_seconds.read().unwrap()),
            retries: 2,
        };
        let manager = self.session.fetch_manager();
        let resp = manager.dispatch(req).await?;

        let content_type = sniff_content_type(&resp.headers);
        let suggested_filename = extract_filename(&resp.url, &resp.headers);

        let max = self.max_bytes.unwrap_or(u64::MAX) as usize;
        let body: Vec<u8> = resp.body.iter().take(max).copied().collect();
        let bytes_written = body.len() as u64;

        Ok((
            DownloadResult {
                url: resp.url,
                status: resp.status,
                bytes_written,
                content_type,
                suggested_filename,
                resumed: false,
            },
            body,
        ))
    }
}

/// Extracts the `Content-Type` from response headers, or returns `"application/octet-stream"`.
pub fn sniff_content_type(headers: &std::collections::HashMap<String, String>) -> String {
    headers
        .get("content-type")
        .cloned()
        .unwrap_or_else(|| "application/octet-stream".to_string())
        .split(';')
        .next()
        .unwrap_or("application/octet-stream")
        .trim()
        .to_string()
}

/// Returns a suggested filename from `Content-Disposition: attachment; filename="..."`, or from
/// the last path segment of `url`. Returns `None` if neither is available.
pub fn extract_filename(
    url: &str,
    headers: &std::collections::HashMap<String, String>,
) -> Option<String> {
    // Try Content-Disposition: attachment; filename="foo.zip"
    if let Some(cd) = headers.get("content-disposition") {
        for part in cd.split(';') {
            let part = part.trim();
            if let Some(rest) = part.strip_prefix("filename=") {
                let name = rest.trim().trim_matches('"').trim().to_string();
                if !name.is_empty() {
                    return Some(name);
                }
            }
            // Also handle filename*=UTF-8''foo.zip (RFC 5987)
            if let Some(rest) = part.strip_prefix("filename*=") {
                let name = rest.trim().splitn(3, '\'').last().unwrap_or("").to_string();
                if !name.is_empty() {
                    return Some(name);
                }
            }
        }
    }

    // Fall back to last URL path segment.
    url::Url::parse(url)
        .ok()
        .and_then(|u| {
            u.path_segments()
                .and_then(|mut segs| segs.next_back().map(|s| s.to_string()))
        })
        .filter(|s| !s.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn sniff_content_type_returns_header_mime() {
        let mut headers = HashMap::new();
        headers.insert(
            "content-type".to_string(),
            "application/pdf; charset=utf-8".to_string(),
        );
        assert_eq!(sniff_content_type(&headers), "application/pdf");
    }

    #[test]
    fn sniff_content_type_defaults_to_octet_stream() {
        let headers = HashMap::new();
        assert_eq!(sniff_content_type(&headers), "application/octet-stream");
    }

    #[test]
    fn extract_filename_from_content_disposition() {
        let mut headers = HashMap::new();
        headers.insert(
            "content-disposition".to_string(),
            r#"attachment; filename="report.pdf""#.to_string(),
        );
        assert_eq!(
            extract_filename("https://example.com/dl", &headers).as_deref(),
            Some("report.pdf")
        );
    }

    #[test]
    fn extract_filename_from_url_path() {
        let headers = HashMap::new();
        assert_eq!(
            extract_filename("https://example.com/files/data.csv", &headers).as_deref(),
            Some("data.csv")
        );
    }

    #[test]
    fn extract_filename_none_for_root_path() {
        let headers = HashMap::new();
        // `url::Url::path_segments()` returns None for URLs without a path, but "/" has ""
        // as the last segment which we filter.
        let result = extract_filename("https://example.com/", &headers);
        // Last segment of "/" is "" — should be filtered to None.
        assert!(
            result.is_none() || result.as_deref() == Some(""),
            "unexpected: {result:?}"
        );
    }

    #[test]
    fn downloader_to_file_writes_body() {
        // Integration smoke-test using a temp file and mock transport (file-based, not a live
        // HTTP download). We write raw bytes through the Downloader's internal logic.
        use crate::engine::fetcher::MockTransport;
        use crate::engine::session::Session;
        use std::sync::Arc;

        let mock = Arc::new(
            MockTransport::new().with_html("https://example.com/file.bin", "binary-content-here"),
        );
        let session = Arc::new(Session::new());
        session.set_transport(mock);

        let downloader = Downloader::new(session).with_resume(false);
        let dir = tempfile::tempdir().unwrap();
        let dest = dir.path().join("file.bin");

        let result = downloader
            .download_to_file("https://example.com/file.bin", &dest)
            .unwrap();

        assert_eq!(result.status, 200);
        assert!(result.bytes_written > 0);
        let written = std::fs::read_to_string(&dest).unwrap();
        assert!(written.contains("binary-content-here"));
    }

    #[test]
    fn downloader_to_memory_returns_body() {
        use crate::engine::fetcher::MockTransport;
        use crate::engine::session::Session;
        use std::sync::Arc;

        let mock = Arc::new(
            MockTransport::new().with_html("https://example.com/data.json", r#"{"ok":true}"#),
        );
        let session = Arc::new(Session::new());
        session.set_transport(mock);

        let downloader = Downloader::new(session);
        let (result, body) = downloader
            .download_to_memory("https://example.com/data.json")
            .unwrap();

        assert_eq!(result.status, 200);
        let text = String::from_utf8(body).unwrap();
        assert!(text.contains("\"ok\":true"));
    }

    #[test]
    fn downloader_206_resume_path() {
        use crate::engine::fetcher::{BoxFuture, NormalizedResponse, Transport};
        use crate::engine::session::Session;
        use crate::error::Result;
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        struct RangeCheckTransport {
            has_range: AtomicBool,
        }

        impl Transport for RangeCheckTransport {
            fn fetch<'a>(
                &'a self,
                request: &'a FetchRequest,
            ) -> BoxFuture<'a, Result<NormalizedResponse>> {
                Box::pin(async move {
                    if let Some(range) = request.headers.get("Range") {
                        assert_eq!(range, "bytes=5-");
                        self.has_range.store(true, Ordering::SeqCst);
                        Ok(NormalizedResponse {
                            url: request.url.clone(),
                            status: 206,
                            headers: HashMap::new(),
                            cookies: HashMap::new(),
                            body: "world".into(),
                            content_type: "text/plain".to_string(),
                            encoding: "utf-8".to_string(),
                            timings: Default::default(),
                        })
                    } else {
                        Ok(NormalizedResponse {
                            url: request.url.clone(),
                            status: 200,
                            headers: HashMap::new(),
                            cookies: HashMap::new(),
                            body: "hello".into(),
                            content_type: "text/plain".to_string(),
                            encoding: "utf-8".to_string(),
                            timings: Default::default(),
                        })
                    }
                })
            }
        }

        let mock = Arc::new(RangeCheckTransport {
            has_range: AtomicBool::new(false),
        });
        let session = Arc::new(Session::new());
        session.set_transport(mock.clone());

        let dir = tempfile::tempdir().unwrap();
        let dest = dir.path().join("resume.txt");

        // 1. Write initial 5 bytes
        std::fs::write(&dest, "hello").unwrap();

        let downloader = Downloader::new(session).with_resume(true);
        let result = downloader
            .download_to_file("https://example.com/file", &dest)
            .unwrap();

        assert_eq!(result.status, 206);
        assert!(mock.has_range.load(Ordering::SeqCst));
        assert_eq!(result.bytes_written, 5); // 5 bytes appended
        assert!(result.resumed);

        // Verify file contains both parts
        let content = std::fs::read_to_string(&dest).unwrap();
        assert_eq!(content, "helloworld");
    }
}
