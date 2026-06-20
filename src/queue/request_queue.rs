use crate::engine::fetcher::FetchRequest;
use crossbeam::queue::SegQueue;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

/// Priority levels for the request queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    High,
    Normal,
    Low,
}

/// A wrapped request entry in the queue, carrying queue metadata.
pub struct QueueItem {
    pub request: FetchRequest,
    pub priority: Priority,
    pub created_at: Instant,
}

/// A priority-aware, thread-safe, lock-free request queue built on `crossbeam::queue::SegQueue`.
pub struct RequestQueue {
    high: SegQueue<QueueItem>,
    normal: SegQueue<QueueItem>,
    low: SegQueue<QueueItem>,
    total_processed: AtomicUsize,
    total_wait_ms: AtomicUsize,
}

/// Snapshot metrics of the request queue.
#[derive(Debug, Clone, Copy)]
pub struct QueueMetrics {
    pub depth: usize,
    pub throughput: usize,
    pub avg_wait_ms: f64,
}

impl RequestQueue {
    /// Creates a new, empty `RequestQueue`.
    pub fn new() -> Self {
        Self {
            high: SegQueue::new(),
            normal: SegQueue::new(),
            low: SegQueue::new(),
            total_processed: AtomicUsize::new(0),
            total_wait_ms: AtomicUsize::new(0),
        }
    }

    /// Pushes a fetch request with a specified priority.
    pub fn push(&self, request: FetchRequest, priority: Priority) {
        let item = QueueItem {
            request,
            priority,
            created_at: Instant::now(),
        };

        match priority {
            Priority::High => self.high.push(item),
            Priority::Normal => self.normal.push(item),
            Priority::Low => self.low.push(item),
        }
    }

    /// Pops the next highest priority request from the queues.
    pub fn pop(&self) -> Option<FetchRequest> {
        let item = self
            .high
            .pop()
            .or_else(|| self.normal.pop())
            .or_else(|| self.low.pop())?;

        let wait_time = item.created_at.elapsed().as_millis() as usize;
        self.total_wait_ms.fetch_add(wait_time, Ordering::Relaxed);
        self.total_processed.fetch_add(1, Ordering::Relaxed);

        Some(item.request)
    }

    /// Returns the combined length of all priority queues.
    pub fn queue_depth(&self) -> usize {
        self.high.len() + self.normal.len() + self.low.len()
    }

    /// Gathers latency and throughput metrics from the queue.
    pub fn metrics(&self) -> QueueMetrics {
        let depth = self.queue_depth();
        let throughput = self.total_processed.load(Ordering::Relaxed);
        let total_wait = self.total_wait_ms.load(Ordering::Relaxed);

        let avg_wait_ms = if throughput > 0 {
            total_wait as f64 / throughput as f64
        } else {
            0.0
        };

        QueueMetrics {
            depth,
            throughput,
            avg_wait_ms,
        }
    }
}

impl Default for RequestQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::time::Duration;

    fn mock_request(url: &str) -> FetchRequest {
        FetchRequest {
            url: url.to_string(),
            tier: crate::engine::fetcher::FetcherTier::Standard,
            browser_profile: None,
            headers: HashMap::new(),
            cookies: HashMap::new(),
            proxy: None,
            timeout: Duration::from_secs(10),
            retries: 2,
            rate_limit_rps: 0.0,
        }
    }

    #[test]
    fn test_priority_drain() {
        let queue = RequestQueue::new();
        queue.push(mock_request("http://low.com"), Priority::Low);
        queue.push(mock_request("http://high.com"), Priority::High);
        queue.push(mock_request("http://normal.com"), Priority::Normal);

        // Pop order should respect priority: High -> Normal -> Low
        let p1 = queue.pop().unwrap();
        assert_eq!(p1.url, "http://high.com");

        let p2 = queue.pop().unwrap();
        assert_eq!(p2.url, "http://normal.com");

        let p3 = queue.pop().unwrap();
        assert_eq!(p3.url, "http://low.com");

        assert!(queue.pop().is_none());
    }
}
