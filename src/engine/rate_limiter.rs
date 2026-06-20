use dashmap::DashMap;
use governor::clock::DefaultClock;
use governor::state::direct::NotKeyed;
use governor::state::InMemoryState;
use governor::{Quota, RateLimiter};
use std::sync::Arc;
use std::time::Duration;

pub type DirectRateLimiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;

/// A thread-safe rate limiter manager that maps host names to individual token bucket rate limiters.
pub struct HostRateLimiter {
    limiters: DashMap<String, Arc<DirectRateLimiter>>,
}

impl HostRateLimiter {
    /// Creates a new `HostRateLimiter`.
    pub fn new() -> Self {
        Self {
            limiters: DashMap::new(),
        }
    }

    /// Acquires a slot for the specified host, blocking asynchronously until a token is available.
    pub async fn wait(&self, host: &str, requests_per_second: f64) {
        if requests_per_second <= 0.0 {
            return;
        }

        let host_str = host.to_string();
        let limiter = self.limiters.entry(host_str).or_insert_with(|| {
            let period = Duration::from_secs_f64(1.0 / requests_per_second);
            // Construct a quota of 1 cell per the calculated period.
            // Allow a burst capacity of 1 so it limits strictly to the duration pacing.
            let quota = Quota::with_period(period)
                .expect("Valid duration for rate limit period")
                .allow_burst(std::num::NonZeroU32::new(1).unwrap());
            Arc::new(RateLimiter::direct(quota))
        });

        // Block asynchronously until the rate limiter allows the next request
        limiter.value().until_ready().await;
    }
}

impl Default for HostRateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn test_rate_limiter_pacing() {
        let rl = HostRateLimiter::new();
        let host = "test-host.com";
        let rps = 10.0; // 1 request every 100ms

        let start = Instant::now();
        rl.wait(host, rps).await; // immediate
        rl.wait(host, rps).await; // 100ms delay
        let elapsed = start.elapsed();

        assert!(
            elapsed >= Duration::from_millis(90),
            "Elapsed time was {:?}",
            elapsed
        );
    }
}
