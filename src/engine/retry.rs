use crate::engine::fetcher::NormalizedResponse;
use crate::error::Result;
use std::collections::HashSet;
use std::time::Duration;

/// Decision returned by [`RetryPolicy::decide`] for a single fetch attempt.
#[derive(Debug, Clone, PartialEq)]
pub enum RetryDecision {
    /// Return the current result to the caller — it either succeeded, is not retryable, or no
    /// retry attempts remain.
    Return,
    /// Wait the given duration, then retry the fetch.
    Retry(Duration),
}

/// Governs which fetch outcomes are retried and how long to wait between attempts.
///
/// A bare "retry on transport error" loop misses the common case of a server responding with a
/// non-2xx status: `429 Too Many Requests` and `5xx` server errors are `Ok(NormalizedResponse)`
/// as far as the [`Transport`](crate::engine::fetcher::Transport) is concerned, not an `Err`. This
/// policy inspects the response status too, and honors a `Retry-After` header when the server
/// sends one instead of using the computed exponential backoff.
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub multiplier: f64,
    pub retryable_statuses: HashSet<u16>,
    pub respect_retry_after: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            base_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(30),
            multiplier: 2.0,
            retryable_statuses: [429, 500, 502, 503, 504].into_iter().collect(),
            respect_retry_after: true,
        }
    }
}

impl RetryPolicy {
    /// A policy that only retries on transport errors, never on response status — restores the
    /// pre-fix behavior for callers that want it.
    pub fn status_retries_disabled() -> Self {
        Self {
            retryable_statuses: HashSet::new(),
            ..Self::default()
        }
    }

    /// Computes the exponential backoff delay for a zero-indexed attempt number, capped at
    /// `max_delay`.
    pub fn backoff_for(&self, attempt: usize) -> Duration {
        let millis = (self.base_delay.as_millis() as f64) * self.multiplier.powi(attempt as i32);
        let capped = millis.min(self.max_delay.as_millis() as f64);
        Duration::from_millis(capped as u64)
    }

    /// Decides whether `result` (the outcome of one fetch attempt) should be retried, given that
    /// `attempt` attempts (zero-indexed) have already been made out of `max_attempts` allowed
    /// retries.
    pub fn decide(
        &self,
        attempt: usize,
        max_attempts: usize,
        result: &Result<NormalizedResponse>,
    ) -> RetryDecision {
        if attempt >= max_attempts {
            return RetryDecision::Return;
        }

        match result {
            Err(_) => RetryDecision::Retry(self.backoff_for(attempt)),
            Ok(response) => {
                if !self.retryable_statuses.contains(&response.status) {
                    return RetryDecision::Return;
                }
                let delay = if self.respect_retry_after {
                    retry_after_delay(response).unwrap_or_else(|| self.backoff_for(attempt))
                } else {
                    self.backoff_for(attempt)
                };
                RetryDecision::Retry(delay)
            }
        }
    }
}

/// Parses a numeric-seconds `Retry-After` header, if present.
///
/// The HTTP-date form of `Retry-After` is not handled; retry falls back to the computed
/// exponential backoff in that case.
fn retry_after_delay(response: &NormalizedResponse) -> Option<Duration> {
    response
        .headers
        .get("retry-after")
        .and_then(|v| v.trim().parse::<u64>().ok())
        .map(Duration::from_secs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::CrawlingoError;
    use bytes::Bytes;
    use std::collections::HashMap;

    fn response_with(status: u16, headers: HashMap<String, String>) -> NormalizedResponse {
        NormalizedResponse {
            url: "https://example.com".to_string(),
            status,
            headers,
            cookies: HashMap::new(),
            body: Bytes::new(),
            content_type: "text/html".to_string(),
            encoding: "utf-8".to_string(),
            timings: Default::default(),
        }
    }

    #[test]
    fn retries_on_default_statuses() {
        let policy = RetryPolicy::default();
        for status in [429, 500, 502, 503, 504] {
            let res: Result<NormalizedResponse> = Ok(response_with(status, HashMap::new()));
            assert_eq!(
                policy.decide(0, 3, &res),
                RetryDecision::Retry(policy.backoff_for(0)),
                "status {status} should be retried"
            );
        }
    }

    #[test]
    fn does_not_retry_client_errors_other_than_429() {
        let policy = RetryPolicy::default();
        let res: Result<NormalizedResponse> = Ok(response_with(404, HashMap::new()));
        assert_eq!(policy.decide(0, 3, &res), RetryDecision::Return);
    }

    #[test]
    fn does_not_retry_success() {
        let policy = RetryPolicy::default();
        let res: Result<NormalizedResponse> = Ok(response_with(200, HashMap::new()));
        assert_eq!(policy.decide(0, 3, &res), RetryDecision::Return);
    }

    #[test]
    fn stops_once_max_attempts_reached() {
        let policy = RetryPolicy::default();
        let res: Result<NormalizedResponse> = Ok(response_with(503, HashMap::new()));
        assert_eq!(policy.decide(3, 3, &res), RetryDecision::Return);
    }

    #[test]
    fn retries_transport_errors_regardless_of_status_set() {
        let policy = RetryPolicy::status_retries_disabled();
        let res: Result<NormalizedResponse> = Err(CrawlingoError::FetchError("boom".to_string()));
        assert_eq!(
            policy.decide(0, 3, &res),
            RetryDecision::Retry(policy.backoff_for(0))
        );
    }

    #[test]
    fn honors_retry_after_header_over_backoff() {
        let policy = RetryPolicy::default();
        let mut headers = HashMap::new();
        headers.insert("retry-after".to_string(), "7".to_string());
        let res: Result<NormalizedResponse> = Ok(response_with(429, headers));
        assert_eq!(
            policy.decide(0, 3, &res),
            RetryDecision::Retry(Duration::from_secs(7))
        );
    }

    #[test]
    fn ignores_retry_after_when_disabled() {
        let mut policy = RetryPolicy::default();
        policy.respect_retry_after = false;
        let mut headers = HashMap::new();
        headers.insert("retry-after".to_string(), "7".to_string());
        let res: Result<NormalizedResponse> = Ok(response_with(429, headers));
        assert_eq!(
            policy.decide(0, 3, &res),
            RetryDecision::Retry(policy.backoff_for(0))
        );
    }

    #[test]
    fn backoff_grows_exponentially_and_caps() {
        let policy = RetryPolicy::default();
        assert_eq!(policy.backoff_for(0), Duration::from_millis(500));
        assert_eq!(policy.backoff_for(1), Duration::from_millis(1000));
        assert_eq!(policy.backoff_for(2), Duration::from_millis(2000));
        // 500 * 2^10 = 512000ms, well past the 30s cap.
        assert_eq!(policy.backoff_for(10), policy.max_delay);
    }
}
