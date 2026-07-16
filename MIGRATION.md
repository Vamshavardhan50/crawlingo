# Migration Guide

## Crawlingo v1.0.0 Migrations

### PersistentFrontier store format (M2)

In Milestone 2, the `PersistentFrontier` was redesigned to avoid serialization overhead and support high-throughput scaling. 

**What Changed:**
- Previously, the entire pending queue was stored as a single serialized `Vec` under the key `__frontier_pending__` (causing $O(n)$ writes and reads).
- Now, queue elements are stored as individual database keys prefixed by `pending:{id:016}` (achieving $O(1)$ enqueue and $O(\log n)$ dequeue), with a monotonic counter key `pending_counter` tracking the insertions.

**Impact:**
Existing `PersistentFrontier` Sled database directories created with pre-Milestone 2 code are incompatible with the new format. Attempting to reopen an old database directory will result in deserialization errors or missing pending queue items.

**Migration Action Required:**
To migrate existing persistent queues to the new format:
1. Delete the old persistent store directory (configured by your crawler/frontier storage path).
2. Allow Crawlingo to recreate the persistent frontier automatically on the next crawl run.

**Example Command (Bash):**
```bash
rm -rf path/to/frontier/db
```

**Example Command (PowerShell):**
```powershell
Remove-Item -Recurse -Force path/to/frontier/db
```

---

### FetchRequest `rate_limit_rps` removal (M3)

**What Changed:**
- The request-level `rate_limit_rps` parameter was removed from `FetchRequest` to simplify the core dispatcher. 
- Rate limiting is now configured at the `Session` level and managed dynamically by `FetchManager` and shared `HostRateLimiter` instances.

**Impact:**
Constructing individual `FetchRequest` structures with the `rate_limit_rps` field will fail to compile.

**Migration Action Required:**
1. Remove `rate_limit_rps` field assignments from any manual `FetchRequest` struct initializations.
2. Set rate limits using `Session::rate_limit` (Python/JS) or `FetchManager::with_rate_limit_rps` (Rust).

---

### `ExtractionRule` -> `DatasetField` type alias (M3)

**What Changed:**
- `ExtractionRule` was converted from a standalone struct into a type alias for `DatasetField` (`pub type ExtractionRule = crate::dataset::builder::DatasetField;`).

**Impact:**
Any legacy code directly calling conversion loops (e.g. converting `ExtractionRule` into `DatasetField`) is no longer necessary.

**Migration Action Required:**
1. Replace structural conversions with direct uses of `DatasetField`.
2. Update references to `crawlingo::extraction::ExtractionRule` accordingly.
