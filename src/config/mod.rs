//! Configuration loading for a [`crate::engine::session::Session`].
//!
//! [`CrawlingoConfig`] mirrors the subset of `Session`'s runtime state that makes sense to set
//! ahead of time from a file or environment, rather than only via the fluent `Session`/`PySession`
//! setters. [`CrawlingoConfig::load`] merges configuration from multiple sources with the
//! following precedence, lowest to highest:
//!
//! 1. **Defaults** — [`CrawlingoConfig::default()`].
//! 2. **Config file** — TOML or JSON, selected by extension (`.toml`/`.json`).
//! 3. **Environment variables** — `CRAWLINGO_*`, see [`CrawlingoConfig::apply_env`].
//!
//! The resulting config is applied to a fresh [`crate::engine::session::Session`] via
//! [`crate::engine::session::Session::from_config`]; explicit `Session` setter calls made
//! afterward always take precedence over anything loaded here.

use crate::error::{CrawlingoError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::time::Duration;

/// Connection-pool settings, mirroring [`crate::engine::pool::ConnectionPoolConfig`] in a
/// serializable form (durations as whole seconds).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct PoolConfigSpec {
    pub max_idle_per_host: usize,
    pub idle_timeout_secs: u64,
    pub tcp_keepalive_secs: u64,
    pub max_clients: u64,
}

impl Default for PoolConfigSpec {
    fn default() -> Self {
        let d = crate::engine::pool::ConnectionPoolConfig::default();
        Self {
            max_idle_per_host: d.max_idle_per_host,
            idle_timeout_secs: d.idle_timeout.as_secs(),
            tcp_keepalive_secs: d.tcp_keepalive.as_secs(),
            max_clients: d.max_clients,
        }
    }
}

impl From<&PoolConfigSpec> for crate::engine::pool::ConnectionPoolConfig {
    fn from(spec: &PoolConfigSpec) -> Self {
        crate::engine::pool::ConnectionPoolConfig {
            max_idle_per_host: spec.max_idle_per_host,
            idle_timeout: Duration::from_secs(spec.idle_timeout_secs),
            tcp_keepalive: Duration::from_secs(spec.tcp_keepalive_secs),
            max_clients: spec.max_clients.max(1),
        }
    }
}

/// Retry policy settings, mirroring [`crate::engine::retry::RetryPolicy`] in a serializable form.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct RetryConfigSpec {
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
    pub multiplier: f64,
    pub retryable_statuses: Vec<u16>,
    pub respect_retry_after: bool,
}

impl Default for RetryConfigSpec {
    fn default() -> Self {
        let d = crate::engine::retry::RetryPolicy::default();
        Self {
            base_delay_ms: d.base_delay.as_millis() as u64,
            max_delay_ms: d.max_delay.as_millis() as u64,
            multiplier: d.multiplier,
            retryable_statuses: d.retryable_statuses.iter().copied().collect(),
            respect_retry_after: d.respect_retry_after,
        }
    }
}

impl From<&RetryConfigSpec> for crate::engine::retry::RetryPolicy {
    fn from(spec: &RetryConfigSpec) -> Self {
        crate::engine::retry::RetryPolicy {
            base_delay: Duration::from_millis(spec.base_delay_ms),
            max_delay: Duration::from_millis(spec.max_delay_ms),
            multiplier: spec.multiplier,
            retryable_statuses: spec.retryable_statuses.iter().copied().collect(),
            respect_retry_after: spec.respect_retry_after,
        }
    }
}

/// Top-level configuration for a [`crate::engine::session::Session`].
///
/// All fields are optional-with-defaults (`#[serde(default)]`) so a config file only needs to
/// specify the values it wants to override.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(default)]
pub struct CrawlingoConfig {
    pub headers: HashMap<String, String>,
    pub proxy: Option<String>,
    pub proxy_pool: Vec<String>,
    pub proxy_provider_url: Option<String>,
    pub rate_limit_rps: f64,
    pub auto_match: bool,
    pub timeout_seconds: u64,
    pub fingerprint_path: Option<String>,
    /// `"standard"` or `"stealthy"`. Mirrors `PySession::fetcher_tier`'s string convention.
    pub fetcher_tier: Option<String>,
    pub browser_profile: Option<String>,
    pub pool: PoolConfigSpec,
    pub retry: RetryConfigSpec,
}

impl CrawlingoConfig {
    /// Loads configuration with the documented precedence: defaults, then an optional config
    /// file, then `CRAWLINGO_*` environment variables.
    ///
    /// `path` is optional — pass `None` to skip the file layer and load from defaults + env only
    /// (the common case for SDK callers who configure everything via `Session` setters and only
    /// want environment-variable overrides, e.g. `CRAWLINGO_PROXY` in a container).
    pub fn load(path: Option<&Path>) -> Result<Self> {
        let mut config = if let Some(path) = path {
            Self::from_file(path)?
        } else {
            Self::default()
        };
        config.apply_env();
        Ok(config)
    }

    /// Loads configuration from a single TOML or JSON file, selected by extension.
    pub fn from_file(path: &Path) -> Result<Self> {
        let contents = std::fs::read_to_string(path).map_err(|e| {
            CrawlingoError::ConfigError(format!(
                "failed to read config file {}: {e}",
                path.display()
            ))
        })?;

        match path.extension().and_then(|e| e.to_str()) {
            Some("toml") => toml::from_str(&contents).map_err(|e| {
                CrawlingoError::ConfigError(format!(
                    "failed to parse TOML config {}: {e}",
                    path.display()
                ))
            }),
            Some("json") => serde_json::from_str(&contents).map_err(|e| {
                CrawlingoError::ConfigError(format!(
                    "failed to parse JSON config {}: {e}",
                    path.display()
                ))
            }),
            other => Err(CrawlingoError::ConfigError(format!(
                "unsupported config file extension {:?} (expected .toml or .json): {}",
                other,
                path.display()
            ))),
        }
    }

    /// Overlays `CRAWLINGO_*` environment variables onto this config in place.
    ///
    /// Recognized variables: `CRAWLINGO_PROXY`, `CRAWLINGO_PROXY_PROVIDER_URL`,
    /// `CRAWLINGO_RATE_LIMIT_RPS`, `CRAWLINGO_AUTO_MATCH`, `CRAWLINGO_TIMEOUT_SECONDS`,
    /// `CRAWLINGO_FINGERPRINT_PATH`, `CRAWLINGO_FETCHER_TIER`, `CRAWLINGO_BROWSER_PROFILE`.
    /// Unset or unparseable variables are left as-is (the existing value is kept); a present but
    /// invalid value is ignored with a `tracing::warn!` rather than failing the whole load, since
    /// environment misconfiguration should degrade gracefully rather than crash the process.
    pub fn apply_env(&mut self) {
        self.apply_env_from(|key| std::env::var(key).ok());
    }

    /// Like [`CrawlingoConfig::apply_env`], but reads variables through a caller-supplied lookup
    /// function instead of the real process environment. Exists so tests can exercise env-layer
    /// precedence deterministically, without mutating (and racing on) global process state.
    pub fn apply_env_from(&mut self, lookup: impl Fn(&str) -> Option<String>) {
        if let Some(v) = lookup("CRAWLINGO_PROXY") {
            self.proxy = Some(v);
        }
        if let Some(v) = lookup("CRAWLINGO_PROXY_PROVIDER_URL") {
            self.proxy_provider_url = Some(v);
        }
        if let Some(v) = lookup("CRAWLINGO_RATE_LIMIT_RPS") {
            match v.parse() {
                Ok(rps) => self.rate_limit_rps = rps,
                Err(_) => tracing::warn!("ignoring invalid CRAWLINGO_RATE_LIMIT_RPS={v:?}"),
            }
        }
        if let Some(v) = lookup("CRAWLINGO_AUTO_MATCH") {
            match v.to_lowercase().as_str() {
                "1" | "true" | "yes" => self.auto_match = true,
                "0" | "false" | "no" => self.auto_match = false,
                _ => tracing::warn!("ignoring invalid CRAWLINGO_AUTO_MATCH={v:?}"),
            }
        }
        if let Some(v) = lookup("CRAWLINGO_TIMEOUT_SECONDS") {
            match v.parse() {
                Ok(secs) => self.timeout_seconds = secs,
                Err(_) => tracing::warn!("ignoring invalid CRAWLINGO_TIMEOUT_SECONDS={v:?}"),
            }
        }
        if let Some(v) = lookup("CRAWLINGO_FINGERPRINT_PATH") {
            self.fingerprint_path = Some(v);
        }
        if let Some(v) = lookup("CRAWLINGO_FETCHER_TIER") {
            self.fetcher_tier = Some(v);
        }
        if let Some(v) = lookup("CRAWLINGO_BROWSER_PROFILE") {
            self.browser_profile = Some(v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap as StdHashMap;

    #[test]
    fn defaults_round_trip_pool_and_retry_specs() {
        let config = CrawlingoConfig::default();
        let pool: crate::engine::pool::ConnectionPoolConfig = (&config.pool).into();
        let default_pool = crate::engine::pool::ConnectionPoolConfig::default();
        assert_eq!(pool.max_idle_per_host, default_pool.max_idle_per_host);
        assert_eq!(pool.max_clients, default_pool.max_clients);

        let retry: crate::engine::retry::RetryPolicy = (&config.retry).into();
        assert_eq!(
            retry.base_delay,
            crate::engine::retry::RetryPolicy::default().base_delay
        );
        assert!(retry.retryable_statuses.contains(&429));
    }

    #[test]
    fn loads_toml_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("crawlingo.toml");
        std::fs::write(
            &path,
            r#"
                proxy = "http://proxy.example.com:8080"
                rate_limit_rps = 2.5
                auto_match = true
                timeout_seconds = 45
                fetcher_tier = "stealthy"
            "#,
        )
        .unwrap();

        let config = CrawlingoConfig::from_file(&path).unwrap();
        assert_eq!(
            config.proxy.as_deref(),
            Some("http://proxy.example.com:8080")
        );
        assert_eq!(config.rate_limit_rps, 2.5);
        assert!(config.auto_match);
        assert_eq!(config.timeout_seconds, 45);
        assert_eq!(config.fetcher_tier.as_deref(), Some("stealthy"));
    }

    #[test]
    fn loads_json_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("crawlingo.json");
        std::fs::write(
            &path,
            r#"{"proxy": "http://p.example.com", "timeout_seconds": 10}"#,
        )
        .unwrap();

        let config = CrawlingoConfig::from_file(&path).unwrap();
        assert_eq!(config.proxy.as_deref(), Some("http://p.example.com"));
        assert_eq!(config.timeout_seconds, 10);
    }

    #[test]
    fn rejects_unsupported_extension() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("crawlingo.yaml");
        std::fs::write(&path, "proxy: nope").unwrap();
        assert!(CrawlingoConfig::from_file(&path).is_err());
    }

    #[test]
    fn missing_file_is_an_error_not_a_silent_default() {
        let path = Path::new("/does/not/exist/crawlingo.toml");
        assert!(CrawlingoConfig::from_file(path).is_err());
    }

    #[test]
    fn env_layer_overrides_file_layer() {
        let mut config = CrawlingoConfig {
            proxy: Some("http://from-file.example.com".to_string()),
            timeout_seconds: 30,
            ..Default::default()
        };

        let mut env: StdHashMap<&str, &str> = StdHashMap::new();
        env.insert("CRAWLINGO_PROXY", "http://from-env.example.com");
        env.insert("CRAWLINGO_RATE_LIMIT_RPS", "9.5");
        config.apply_env_from(|key| env.get(key).map(|v| v.to_string()));

        assert_eq!(config.proxy.as_deref(), Some("http://from-env.example.com"));
        assert_eq!(config.rate_limit_rps, 9.5);
        // Untouched by env — file value survives.
        assert_eq!(config.timeout_seconds, 30);
    }

    #[test]
    fn invalid_env_value_is_ignored_not_fatal() {
        let mut config = CrawlingoConfig {
            rate_limit_rps: 1.0,
            ..Default::default()
        };
        let mut env: StdHashMap<&str, &str> = StdHashMap::new();
        env.insert("CRAWLINGO_RATE_LIMIT_RPS", "not-a-number");
        config.apply_env_from(|key| env.get(key).map(|v| v.to_string()));
        // Invalid value is ignored; original default is preserved rather than the load failing.
        assert_eq!(config.rate_limit_rps, 1.0);
    }
}
