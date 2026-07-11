//! The crawl frontier: the pending-URL queue and visited-set a [`crate::crawl::crawler::Crawler`]
//! consults to decide what to fetch next.
//!
//! [`MemoryFrontier`] is the default — an in-process queue with no persistence, exactly matching
//! the crawler's original behavior (a per-run `Vec` stack + `HashSet`). [`PersistentFrontier`]
//! backs the same state with `sled` so a crawl can be resumed after a process restart: reopening
//! the same path picks up wherever the previous run left off, instead of re-crawling from the seed
//! URL.

use crate::error::{CrawlingoError, Result};
use std::collections::HashSet;
use std::path::Path;
use std::sync::Mutex;

/// The pending-URL queue and visited-set consulted by a crawl.
///
/// Methods are synchronous (not `async`) even though [`PersistentFrontier`] does file I/O — sled
/// itself is a blocking embedded database, so there is no async operation to await; callers on an
/// async worker loop simply call these directly without `.await`, which is both simpler and
/// avoids holding a lock across an await point.
pub trait Frontier: Send + Sync {
    /// Adds `url` to the pending queue at `depth`, unless it's already been visited.
    fn enqueue(&self, url: String, depth: usize);
    /// Pops the next `(url, depth)` pair to crawl, if any remain.
    fn dequeue(&self) -> Option<(String, usize)>;
    /// Marks `url` as visited (crawled, or claimed by a worker) so it won't be re-enqueued.
    fn mark_visited(&self, url: &str);
    /// Whether `url` has already been visited.
    fn is_visited(&self, url: &str) -> bool;
    /// Number of items currently pending.
    fn pending_len(&self) -> usize;
    /// Number of URLs visited so far.
    fn visited_len(&self) -> usize;
}

/// The default, non-persistent [`Frontier`] — an in-memory LIFO queue plus a visited set, scoped
/// to a single crawl run. Matches the crawler's original (pre-`Frontier`) behavior exactly.
#[derive(Default)]
pub struct MemoryFrontier {
    pending: Mutex<Vec<(String, usize)>>,
    visited: Mutex<HashSet<String>>,
}

impl MemoryFrontier {
    pub fn new() -> Self {
        Self::default()
    }

    /// Seeds the frontier with a starting URL at depth 0. Chainable.
    pub fn with_seed(self, url: impl Into<String>) -> Self {
        self.enqueue(url.into(), 0);
        self
    }
}

impl Frontier for MemoryFrontier {
    fn enqueue(&self, url: String, depth: usize) {
        if !self.is_visited(&url) {
            self.pending.lock().unwrap().push((url, depth));
        }
    }

    fn dequeue(&self) -> Option<(String, usize)> {
        self.pending.lock().unwrap().pop()
    }

    fn mark_visited(&self, url: &str) {
        self.visited.lock().unwrap().insert(url.to_string());
    }

    fn is_visited(&self, url: &str) -> bool {
        self.visited.lock().unwrap().contains(url)
    }

    fn pending_len(&self) -> usize {
        self.pending.lock().unwrap().len()
    }

    fn visited_len(&self) -> usize {
        self.visited.lock().unwrap().len()
    }
}

/// A `sled`-backed [`Frontier`] that survives a process restart.
///
/// **Scaling note:** the pending queue is stored as a single serialized `Vec` under one key,
/// rewritten whole on every `enqueue`/`dequeue` — simple and correct, but `O(n)` per operation.
/// Fine for the thousands-of-URLs crawls this targets; a frontier meant for millions of pending
/// URLs would need a real on-disk queue (e.g. keyed by a monotonic id) instead. The visited set,
/// by contrast, is a proper sled key per URL and stays `O(1)`.
pub struct PersistentFrontier {
    db: sled::Db,
}

const PENDING_KEY: &[u8] = b"__frontier_pending__";
const VISITED_PREFIX: &str = "visited:";

impl PersistentFrontier {
    /// Opens (or creates) the frontier database at `path`. Reopening the same path resumes
    /// whatever pending/visited state a previous run left behind.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db = sled::open(path).map_err(|e| {
            CrawlingoError::FingerprintStoreError(format!("failed to open frontier store: {e}"))
        })?;
        Ok(Self { db })
    }

    /// Whether this frontier has no pending work and nothing visited yet — i.e. a fresh store
    /// with no prior run's state, as opposed to one that's merely exhausted its queue.
    pub fn is_empty(&self) -> bool {
        self.pending_len() == 0 && self.visited_len() == 0
    }

    fn load_pending(&self) -> Vec<(String, usize)> {
        self.db
            .get(PENDING_KEY)
            .ok()
            .flatten()
            .and_then(|ivec| bincode::deserialize(&ivec).ok())
            .unwrap_or_default()
    }

    fn save_pending(&self, pending: &[(String, usize)]) {
        if let Ok(bytes) = bincode::serialize(pending) {
            let _ = self.db.insert(PENDING_KEY, bytes);
        }
    }
}

impl Frontier for PersistentFrontier {
    fn enqueue(&self, url: String, depth: usize) {
        if self.is_visited(&url) {
            return;
        }
        let mut pending = self.load_pending();
        pending.push((url, depth));
        self.save_pending(&pending);
    }

    fn dequeue(&self) -> Option<(String, usize)> {
        let mut pending = self.load_pending();
        let item = pending.pop();
        self.save_pending(&pending);
        item
    }

    fn mark_visited(&self, url: &str) {
        let key = format!("{VISITED_PREFIX}{url}");
        let _ = self.db.insert(key.as_bytes(), &[]);
    }

    fn is_visited(&self, url: &str) -> bool {
        let key = format!("{VISITED_PREFIX}{url}");
        self.db.contains_key(key.as_bytes()).unwrap_or(false)
    }

    fn pending_len(&self) -> usize {
        self.load_pending().len()
    }

    fn visited_len(&self) -> usize {
        self.db.scan_prefix(VISITED_PREFIX.as_bytes()).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_frontier_enqueue_dequeue_and_visited_tracking() {
        let frontier = MemoryFrontier::new();
        frontier.enqueue("https://a.example.com".to_string(), 0);
        frontier.enqueue("https://b.example.com".to_string(), 1);
        assert_eq!(frontier.pending_len(), 2);

        // LIFO order, matching the crawler's original Vec::pop() behavior.
        let (url, depth) = frontier.dequeue().unwrap();
        assert_eq!(url, "https://b.example.com");
        assert_eq!(depth, 1);

        frontier.mark_visited("https://b.example.com");
        assert!(frontier.is_visited("https://b.example.com"));
        assert!(!frontier.is_visited("https://a.example.com"));
        assert_eq!(frontier.visited_len(), 1);
    }

    #[test]
    fn memory_frontier_does_not_reenqueue_a_visited_url() {
        let frontier = MemoryFrontier::new();
        frontier.mark_visited("https://seen.example.com");
        frontier.enqueue("https://seen.example.com".to_string(), 0);
        assert_eq!(
            frontier.pending_len(),
            0,
            "already-visited URLs must not be enqueued"
        );
    }

    #[test]
    fn with_seed_enqueues_at_depth_zero() {
        let frontier = MemoryFrontier::new().with_seed("https://start.example.com");
        assert_eq!(frontier.pending_len(), 1);
        assert_eq!(
            frontier.dequeue(),
            Some(("https://start.example.com".to_string(), 0))
        );
    }

    #[test]
    fn persistent_frontier_survives_reopening_the_same_path() {
        let dir = tempfile::tempdir().unwrap();

        {
            let frontier = PersistentFrontier::open(dir.path()).unwrap();
            assert!(frontier.is_empty());
            frontier.enqueue("https://a.example.com".to_string(), 0);
            frontier.enqueue("https://b.example.com".to_string(), 1);
            frontier.mark_visited("https://already-done.example.com");
            // Dropped here — simulates the process exiting mid-crawl.
        }

        let reopened = PersistentFrontier::open(dir.path()).unwrap();
        assert!(!reopened.is_empty());
        assert_eq!(reopened.pending_len(), 2);
        assert!(reopened.is_visited("https://already-done.example.com"));
        assert_eq!(
            reopened.dequeue(),
            Some(("https://b.example.com".to_string(), 1)),
            "pending queue order must survive the reopen"
        );
    }

    #[test]
    fn persistent_frontier_does_not_reenqueue_a_visited_url() {
        let dir = tempfile::tempdir().unwrap();
        let frontier = PersistentFrontier::open(dir.path()).unwrap();
        frontier.mark_visited("https://seen.example.com");
        frontier.enqueue("https://seen.example.com".to_string(), 0);
        assert_eq!(frontier.pending_len(), 0);
    }
}
