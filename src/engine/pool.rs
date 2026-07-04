use std::time::Duration;

/// Default upper bound on the number of distinct HTTP clients kept in the fetcher's client cache.
///
/// One client is created per distinct transport configuration (proxy × tier × browser profile).
/// This cap prevents unbounded growth when, for example, a large rotating proxy pool is used;
/// the least-recently-used clients are evicted (and their idle connections dropped) beyond it.
pub const DEFAULT_MAX_CLIENTS: u64 = 128;

/// Configuration options for the HTTP client connection pool.
///
/// These settings are applied when constructing each cached [`wreq::Client`], so they directly
/// control how connections are reused:
/// - `max_idle_per_host` — how many idle keep-alive connections are retained per host. `0`
///   disables keep-alive, forcing a fresh connection per request.
/// - `idle_timeout` — how long an idle connection may live before being closed.
/// - `tcp_keepalive` — TCP-level keepalive probe interval for established connections.
/// - `max_clients` — upper bound on the number of distinct clients cached (see
///   [`DEFAULT_MAX_CLIENTS`]).
///
/// [`wreq::Client`]: https://docs.rs/wreq
#[derive(Debug, Clone)]
pub struct ConnectionPoolConfig {
    pub max_idle_per_host: usize,
    pub idle_timeout: Duration,
    pub tcp_keepalive: Duration,
    pub max_clients: u64,
}

impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self {
            max_idle_per_host: 100,
            idle_timeout: Duration::from_secs(90),
            tcp_keepalive: Duration::from_secs(30),
            max_clients: DEFAULT_MAX_CLIENTS,
        }
    }
}

impl ConnectionPoolConfig {
    /// Creates a custom connection pool config with the default client-cache bound.
    pub fn new(max_idle: usize, idle_timeout_secs: u64, keepalive_secs: u64) -> Self {
        Self {
            max_idle_per_host: max_idle,
            idle_timeout: Duration::from_secs(idle_timeout_secs),
            tcp_keepalive: Duration::from_secs(keepalive_secs),
            max_clients: DEFAULT_MAX_CLIENTS,
        }
    }

    /// Sets the maximum number of distinct clients retained in the cache. Chainable.
    pub fn with_max_clients(mut self, max_clients: u64) -> Self {
        self.max_clients = max_clients.max(1);
        self
    }

    /// Disable connection keep-alive entirely (sets max_idle_per_host to 0).
    /// Useful for test environments or short-lived requests where
    /// connection reuse causes issues with closing servers.
    pub fn no_keepalive() -> Self {
        Self {
            max_idle_per_host: 0,
            idle_timeout: Duration::from_secs(5),
            tcp_keepalive: Duration::from_secs(5),
            max_clients: DEFAULT_MAX_CLIENTS,
        }
    }
}
