use super::Frontier;
use crate::error::{CrawlingoError, Result};
use crate::util::url::canonicalize_url;
use std::path::Path;
use std::sync::Mutex;

struct State {
    pending: usize,
    inflight: usize,
    visited: usize,
    error: Option<String>,
}

/// Indexed persistent frontier. All state transitions use an atomic Sled batch and
/// a per-instance lock; URL membership and queue lengths never scan the queue.
pub struct PersistentFrontier {
    db: sled::Db,
    state: Mutex<State>,
}
impl PersistentFrontier {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let db = sled::Config::new()
            .path(path)
            .cache_capacity(32 * 1024 * 1024)
            .open()?;
        // Migrate the old queue lazily; preserve the original pending ordering.
        if !db.contains_key(b"frontier_index_v2")? {
            for item in db.scan_prefix(b"pending:") {
                let (key, value) = item?;
                let (url, _): (String, usize) = bincode::deserialize(&value)?;
                db.insert(format!("queued:{url}"), key)?;
            }
            db.insert(b"frontier_index_v2", &[])?;
        }
        // Replay interrupted work atomically, one item at a time (bounded memory).
        for item in db.scan_prefix(b"inflight:") {
            let (key, value) = item?;
            let (url, _): (String, usize) = bincode::deserialize(&value)?;
            let mut batch = sled::Batch::default();
            if !db.contains_key(format!("visited:{url}"))?
                && !db.contains_key(format!("queued:{url}"))?
            {
                let pending = format!("pending:{:020}", db.generate_id()?);
                batch.insert(pending.as_bytes(), value);
                batch.insert(format!("queued:{url}").as_bytes(), pending.as_bytes());
            }
            batch.remove(key);
            db.apply_batch(batch)?;
        }
        db.flush()?;
        let state = State {
            pending: db.scan_prefix(b"pending:").count(),
            inflight: 0,
            visited: db.scan_prefix(b"visited:").count(),
            error: None,
        };
        Ok(Self {
            db,
            state: Mutex::new(state),
        })
    }
    pub fn is_empty(&self) -> bool {
        let s = self.state.lock().unwrap();
        s.pending + s.inflight + s.visited == 0
    }
    pub fn flush(&self) -> Result<()> {
        self.check_error()?;
        self.db.flush()?;
        Ok(())
    }
    fn record_error(state: &mut State, e: CrawlingoError) {
        if state.error.is_none() {
            state.error = Some(e.to_string());
        }
    }
    fn canonical(url: &str) -> String {
        canonicalize_url(url).unwrap_or_else(|| url.to_string())
    }
}
impl Frontier for PersistentFrontier {
    fn check_error(&self) -> Result<()> {
        match &self.state.lock().unwrap().error {
            Some(e) => Err(CrawlingoError::FingerprintStoreError(e.clone())),
            None => Ok(()),
        }
    }
    fn enqueue(&self, url: String, depth: usize) {
        let url = Self::canonical(&url);
        let mut state = self.state.lock().unwrap();
        let result = (|| -> Result<()> {
            for prefix in ["visited:", "inflight:", "queued:"] {
                if self.db.contains_key(format!("{prefix}{url}"))? {
                    return Ok(());
                }
            }
            let key = format!("pending:{:020}", self.db.generate_id()?);
            let mut batch = sled::Batch::default();
            batch.insert(key.as_bytes(), bincode::serialize(&(url.clone(), depth))?);
            batch.insert(format!("queued:{url}").as_bytes(), key.as_bytes());
            self.db.apply_batch(batch)?;
            state.pending += 1;
            Ok(())
        })();
        if let Err(e) = result {
            Self::record_error(&mut state, e);
        }
    }
    fn dequeue(&self) -> Option<(String, usize)> {
        let mut state = self.state.lock().unwrap();
        let result = (|| -> Result<Option<(String, usize)>> {
            let Some(item) = self.db.scan_prefix(b"pending:").next_back() else {
                return Ok(None);
            };
            let (key, value) = item?;
            let task: (String, usize) = bincode::deserialize(&value)?;
            let mut batch = sled::Batch::default();
            batch.remove(key);
            batch.remove(format!("queued:{}", task.0).as_bytes());
            batch.insert(format!("inflight:{}", task.0).as_bytes(), value);
            self.db.apply_batch(batch)?;
            state.pending -= 1;
            state.inflight += 1;
            Ok(Some(task))
        })();
        match result {
            Ok(task) => task,
            Err(e) => {
                Self::record_error(&mut state, e);
                None
            }
        }
    }
    fn mark_in_flight(&self, url: &str, depth: usize) {
        let url = Self::canonical(url);
        let mut state = self.state.lock().unwrap();
        let result = (|| -> Result<()> {
            let previous = self.db.insert(
                format!("inflight:{url}"),
                bincode::serialize(&(url, depth))?,
            )?;
            if previous.is_none() {
                state.inflight += 1;
            }
            Ok(())
        })();
        if let Err(e) = result {
            Self::record_error(&mut state, e);
        }
    }
    fn unmark_in_flight(&self, url: &str) {
        let mut state = self.state.lock().unwrap();
        match self.db.remove(format!("inflight:{}", Self::canonical(url))) {
            Ok(Some(_)) => state.inflight -= 1,
            Ok(None) => {}
            Err(e) => Self::record_error(&mut state, e.into()),
        }
    }
    fn mark_visited(&self, url: &str) {
        let url = Self::canonical(url);
        let mut state = self.state.lock().unwrap();
        let result = (|| -> Result<()> {
            let inflight = format!("inflight:{url}");
            let visited = format!("visited:{url}");
            let had_inflight = self.db.contains_key(&inflight)?;
            let had_visited = self.db.contains_key(&visited)?;
            let queued = format!("queued:{url}");
            let pending = self.db.get(&queued)?;
            let mut batch = sled::Batch::default();
            batch.remove(inflight.as_bytes());
            batch.insert(visited.as_bytes(), &[]);
            if let Some(key) = pending.as_ref() {
                batch.remove(key.clone());
                batch.remove(queued.as_bytes());
            }
            self.db.apply_batch(batch)?;
            if had_inflight {
                state.inflight -= 1;
            }
            if !had_visited {
                state.visited += 1;
            }
            if pending.is_some() {
                state.pending -= 1;
            }
            Ok(())
        })();
        if let Err(e) = result {
            Self::record_error(&mut state, e);
        }
    }
    fn is_visited(&self, url: &str) -> bool {
        let url = Self::canonical(url);
        let mut state = self.state.lock().unwrap();
        let result = (|| -> Result<bool> {
            Ok(self.db.contains_key(format!("visited:{url}"))?
                || self.db.contains_key(format!("inflight:{url}"))?)
        })();
        match result {
            Ok(found) => found,
            Err(e) => {
                Self::record_error(&mut state, e);
                false
            }
        }
    }
    fn pending_len(&self) -> usize {
        self.state.lock().unwrap().pending
    }
    fn in_flight_len(&self) -> usize {
        self.state.lock().unwrap().inflight
    }
    fn visited_len(&self) -> usize {
        self.state.lock().unwrap().visited
    }
}
impl Drop for PersistentFrontier {
    fn drop(&mut self) {
        if let Err(e) = self.db.flush() {
            tracing::error!("frontier flush failed: {e}");
        }
    }
}
