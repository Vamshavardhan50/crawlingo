//! The crawl frontier: the pending-URL queue and visited-set a [`crate::crawl::crawler::Crawler`]
//! consults to decide what to fetch next.
//!
//! [`MemoryFrontier`] is the default — an in-process queue with no persistence, exactly matching
//! the crawler's original behavior (a per-run `Vec` stack + `HashSet`). [`PersistentFrontier`]
//! backs the same state with `sled` so a crawl can be resumed after a process restart: reopening
//! the same path picks up wherever the previous run left off, instead of re-crawling from the seed
//! URL.

use crate::error::Result;
use crate::util::url::canonicalize_url;
use std::collections::HashSet;

use std::sync::Mutex;

/// The pending-URL queue, visited set, and in-flight tracker consulted by a crawl.
pub trait Frontier: Send + Sync {
    /// Surfaces storage errors from legacy infallible state-transition methods.
    fn check_error(&self) -> Result<()> {
        Ok(())
    }
    /// Adds `url` to the pending queue at `depth`, unless it's already been visited.
    fn enqueue(&self, url: String, depth: usize);
    /// Pops the next `(url, depth)` pair to crawl, if any remain.
    fn dequeue(&self) -> Option<(String, usize)>;
    /// Marks `url` as currently being processed by a worker worker.
    fn mark_in_flight(&self, url: &str, depth: usize);
    /// Removes `url` from the in-flight processing set.
    fn unmark_in_flight(&self, url: &str);
    /// Marks `url` as visited (crawled or finalized) so it won't be re-enqueued.
    fn mark_visited(&self, url: &str);
    /// Whether `url` has already been visited or is in-flight.
    fn is_visited(&self, url: &str) -> bool;
    /// Number of items currently pending.
    fn pending_len(&self) -> usize;
    /// Number of items currently in-flight.
    fn in_flight_len(&self) -> usize;
    /// Number of URLs visited so far.
    fn visited_len(&self) -> usize;
}

/// The default, non-persistent [`Frontier`] — an in-memory LIFO queue plus visited and in-flight sets.
#[derive(Default)]
pub struct MemoryFrontier {
    state: Mutex<MemoryState>,
}

#[derive(Default)]
struct MemoryState {
    pending: Vec<(String, usize)>,
    queued: HashSet<String>,
    visited: HashSet<String>,
    in_flight: HashSet<String>,
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
        let canonical = canonicalize_url(&url).unwrap_or(url);
        let mut state = self.state.lock().unwrap();
        if !state.visited.contains(&canonical)
            && !state.in_flight.contains(&canonical)
            && state.queued.insert(canonical.clone())
        {
            state.pending.push((canonical, depth));
        }
    }

    fn dequeue(&self) -> Option<(String, usize)> {
        let mut state = self.state.lock().unwrap();
        while let Some((url, depth)) = state.pending.pop() {
            state.queued.remove(&url);
            if !state.visited.contains(&url) && state.in_flight.insert(url.clone()) {
                return Some((url, depth));
            }
        }
        None
    }

    fn mark_in_flight(&self, url: &str, _depth: usize) {
        let canonical = canonicalize_url(url).unwrap_or_else(|| url.to_string());
        self.state.lock().unwrap().in_flight.insert(canonical);
    }

    fn unmark_in_flight(&self, url: &str) {
        let canonical = canonicalize_url(url).unwrap_or_else(|| url.to_string());
        self.state.lock().unwrap().in_flight.remove(&canonical);
    }

    fn mark_visited(&self, url: &str) {
        let canonical = canonicalize_url(url).unwrap_or_else(|| url.to_string());
        let mut state = self.state.lock().unwrap();
        state.in_flight.remove(&canonical);
        state.visited.insert(canonical);
    }

    fn is_visited(&self, url: &str) -> bool {
        let canonical = canonicalize_url(url).unwrap_or_else(|| url.to_string());
        let state = self.state.lock().unwrap();
        state.visited.contains(&canonical) || state.in_flight.contains(&canonical)
    }

    fn pending_len(&self) -> usize {
        self.state.lock().unwrap().pending.len()
    }

    fn in_flight_len(&self) -> usize {
        self.state.lock().unwrap().in_flight.len()
    }

    fn visited_len(&self) -> usize {
        self.state.lock().unwrap().visited.len()
    }
}

#[path = "persistent_frontier.rs"]
mod persistent;
pub use persistent::PersistentFrontier;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_frontier_enqueue_dequeue_and_visited_tracking() {
        let frontier = MemoryFrontier::new();
        frontier.enqueue("https://a.example.com".to_string(), 0);
        frontier.enqueue("https://b.example.com".to_string(), 1);
        assert_eq!(frontier.pending_len(), 2);

        let (url, depth) = frontier.dequeue().unwrap();
        assert_eq!(url, "https://b.example.com/");
        assert_eq!(depth, 1);

        frontier.mark_visited("https://b.example.com/");
        assert!(frontier.is_visited("https://b.example.com/"));
        assert!(!frontier.is_visited("https://a.example.com/"));
        assert_eq!(frontier.visited_len(), 1);
    }

    #[test]
    fn memory_frontier_canonicalizes_url_and_deduplicates_fragments() {
        let frontier = MemoryFrontier::new();
        frontier.enqueue("https://example.com/page#section1".to_string(), 0);
        assert_eq!(frontier.pending_len(), 1);

        // Attempting to enqueue URL with different fragment must be deduplicated while first is pending/in-flight
        frontier.enqueue("https://example.com/page#section2".to_string(), 0);
        assert_eq!(frontier.pending_len(), 1);

        let (url, _) = frontier.dequeue().unwrap();
        assert_eq!(url, "https://example.com/page");

        // Now that it's in-flight, enqueueing section3 should also be deduplicated
        frontier.enqueue("https://example.com/page#section3".to_string(), 0);
        assert_eq!(frontier.pending_len(), 0);
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
    fn persistent_frontier_recovers_in_flight_tasks_on_restart() {
        let dir = tempfile::tempdir().unwrap();

        {
            let frontier = PersistentFrontier::open(dir.path()).unwrap();
            frontier.enqueue("https://a.example.com".to_string(), 0);
            frontier.enqueue("https://b.example.com".to_string(), 1);

            let (url, _) = frontier.dequeue().unwrap();
            assert_eq!(url, "https://b.example.com/");
            assert_eq!(frontier.in_flight_len(), 1);
        }

        // Reopen database — crashed in-flight task must be re-enqueued automatically
        let reopened = PersistentFrontier::open(dir.path()).unwrap();
        assert_eq!(reopened.pending_len(), 2);
        assert_eq!(reopened.in_flight_len(), 0);
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
        }

        let reopened = PersistentFrontier::open(dir.path()).unwrap();
        assert!(!reopened.is_empty());
        assert_eq!(reopened.pending_len(), 2);
        assert!(reopened.is_visited("https://already-done.example.com"));
        assert_eq!(
            reopened.dequeue(),
            Some(("https://b.example.com/".to_string(), 1))
        );
    }
}
