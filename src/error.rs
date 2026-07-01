use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrawlingoError {
    #[error("Fetch failed: {0}")]
    FetchError(String),

    #[error("Parse failed: {0}")]
    ParseError(String),

    #[error("Selector failed: {0}")]
    SelectorError(String),

    #[error("Auto-match failed: no element found above threshold")]
    AutoMatchFailed,

    #[error("Timeout after {seconds}s")]
    TimeoutError { seconds: u64 },

    #[error("Rate limit reached for host: {host}")]
    RateLimitError { host: String },

    #[error("Change detection failed: {0}")]
    ChangeDetectionError(String),

    #[error("Dataset error: {0}")]
    DatasetError(String),

    #[error("Export failed: {0}")]
    ExportError(String),

    #[error("DNS resolution failed: {0}")]
    DnsError(String),

    #[error("Fingerprint store error: {0}")]
    FingerprintStoreError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("HTTP client error: {0}")]
    HttpClientError(String),

    #[error("Sled DB error: {0}")]
    SledError(#[from] sled::Error),

    #[error("Bincode error: {0}")]
    BincodeError(#[from] bincode::Error),

    #[error("CSV write error: {0}")]
    CsvError(#[from] csv::Error),

    #[error("Parquet error: {0}")]
    ParquetError(#[from] parquet::errors::ParquetError),

    #[error("Arrow error: {0}")]
    ArrowError(#[from] arrow::error::ArrowError),
}

// Convert FFI errors if python feature is enabled
#[cfg(feature = "python")]
impl From<CrawlingoError> for pyo3::PyErr {
    fn from(err: CrawlingoError) -> pyo3::PyErr {
        pyo3::exceptions::PyRuntimeError::new_err(err.to_string())
    }
}

pub type Result<T, E = CrawlingoError> = std::result::Result<T, E>;
