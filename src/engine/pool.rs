use std::time::Duration;

/// Configuration options for the per-host connection pool managed by wreq/reqwest.
#[derive(Debug, Clone)]
pub struct ConnectionPoolConfig {
    pub max_idle_per_host: usize,
    pub idle_timeout: Duration,
    pub tcp_keepalive: Duration,
}

impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self {
            max_idle_per_host: 100,
            idle_timeout: Duration::from_secs(90),
            tcp_keepalive: Duration::from_secs(30),
        }
    }
}

impl ConnectionPoolConfig {
    /// Creates a custom connection pool config.
    pub fn new(max_idle: usize, idle_timeout_secs: u64, keepalive_secs: u64) -> Self {
        Self {
            max_idle_per_host: max_idle,
            idle_timeout: Duration::from_secs(idle_timeout_secs),
            tcp_keepalive: Duration::from_secs(keepalive_secs),
        }
    }

    /// Disable connection keep-alive entirely (sets max_idle_per_host to 0).
    /// Useful for test environments or short-lived requests where
    /// connection reuse causes issues with closing servers.
    pub fn no_keepalive() -> Self {
        Self {
            max_idle_per_host: 0,
            idle_timeout: Duration::from_secs(5),
            tcp_keepalive: Duration::from_secs(5),
        }
    }
}
