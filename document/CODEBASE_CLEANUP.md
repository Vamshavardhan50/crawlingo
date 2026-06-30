# Codebase Cleanup Audit

This document identifies unused, duplicate, experimental, and generated files in the Crawlingo repository that are candidates for removal or isolation.

---

## 1. Unused & Abandoned Files
- **`src/queue/request_queue.rs`**
  - *Type:* Abandoned Rust Module.
  - *Description:* Implements a multi-priority thread-safe queue (`RequestQueue`) that is never imported or used by any worker engine (Crawler or Dataset).
  - *Action:* Remove the file and clear its module registration in `src/lib.rs`.

---

## 2. Duplicate Documentation Files
- **`document/DATASET_ENGINE.md`**
  - *Type:* Duplicate Documentation.
  - *Description:* A redundant copy of the dataset engine documentation, which is already correctly cataloged as [document/28_DATASET_ENGINE.md](file:///d:/Scraper/document/28_DATASET_ENGINE.md) and [docs/DATASET_ENGINE.md](file:///d:/Scraper/docs/DATASET_ENGINE.md).
  - *Action:* Remove this duplicate file.

---

## 3. Experimental Code & Scratch Scripts
- **`test-crawlingo-install/diag.js`**
- **`test-crawlingo-install/test.js`**
- **`test-crawlingo-install/test_local.js`**
  - *Type:* Scratch testing files.
  - *Description:* Scripts used for manual verification during development. They are not part of the library package or integration test suite.
  - *Action:* Move these scratch files to a `.gitignore`-d directory (e.g. `scratch/`) or delete them.

---

## 4. Generated Output Files Accidentally Committed
- **`waitlist.csv`** (in repository root)
- **`test-crawlingo-install/exported_data.csv`**
- **`test-crawlingo-install/exported_data.json`**
- **`test-crawlingo-install/amazon_featured.csv`**
- **`test-crawlingo-install/amazon_featured.json`**
- **`test-crawlingo-install/diag_html.txt`**
  - *Type:* Generated runtime outputs.
  - *Description:* CSV, JSON, and text dumps generated from scratch script runs.
  - *Action:* Delete these files and ensure their patterns are covered in `.gitignore`.
