use std::net::IpAddr;
use std::time::Duration;
use moka::future::Cache;
use hickory_resolver::TokioAsyncResolver;
use crate::error::{CrawlingoError, Result};

/// A high-performance DNS cache using `moka` and `hickory-resolver`.
pub struct DnsCache {
    cache: Cache<String, IpAddr>,
    resolver: TokioAsyncResolver,
}

impl DnsCache {
    /// Creates a new `DnsCache` with the specified TTL in seconds.
    pub fn new(ttl_seconds: u64) -> Self {
        let cache = Cache::builder()
            .time_to_live(Duration::from_secs(ttl_seconds))
            .build();

        // Attempt system configuration, fallback to default public resolvers
        let resolver = TokioAsyncResolver::tokio_from_system_conf()
            .unwrap_or_else(|_| {
                let config = hickory_resolver::config::ResolverConfig::cloudflare();
                let opts = hickory_resolver::config::ResolverOpts::default();
                TokioAsyncResolver::tokio(config, opts)
            });

        Self { cache, resolver }
    }

    /// Resolves a host name to an IP Address, utilizing the cache.
    pub async fn resolve(&self, host: &str) -> Result<IpAddr> {
        let host_str = host.to_string();
        if let Some(ip) = self.cache.get(&host_str).await {
            return Ok(ip);
        }

        // Cache miss: run async DNS resolution
        let response = self.resolver.lookup_ip(host)
            .await
            .map_err(|e| CrawlingoError::DnsError(e.to_string()))?;

        if let Some(ip) = response.iter().next() {
            self.cache.insert(host_str, ip).await;
            Ok(ip)
        } else {
            Err(CrawlingoError::DnsError(format!("No IP found for host: {}", host)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dns_cache_resolution() {
        let cache = DnsCache::new(60);
        let ip = cache.resolve("example.com").await;
        assert!(ip.is_ok());
        
        // Secondary lookup should hit cache
        let ip2 = cache.resolve("example.com").await;
        assert_eq!(ip.unwrap(), ip2.unwrap());
    }
}
