use std::path::Path;
use crate::error::{CrawlingoError, Result};
use crate::fingerprint::dom::DomFingerprint;

/// Persistent database store for HTML fingerprints using `sled`.
pub struct FingerprintStore {
    db: sled::Db,
}

impl FingerprintStore {
    /// Opens or creates the persistent database at the specified directory path.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db = sled::open(path)
            .map_err(|e| CrawlingoError::FingerprintStoreError(e.to_string()))?;
        Ok(Self { db })
    }

    /// Stores a DOM Fingerprint associated with a URL and selector.
    pub fn store(&self, url: &str, selector: &str, fingerprint: &DomFingerprint) -> Result<()> {
        let key = format!("{}:{}", url, selector);
        let bytes = bincode::serialize(fingerprint)
            .map_err(|e| CrawlingoError::BincodeError(e))?;
        self.db.insert(key.as_bytes(), bytes)
            .map_err(|e| CrawlingoError::FingerprintStoreError(e.to_string()))?;
        self.db.flush()
            .map_err(|e| CrawlingoError::FingerprintStoreError(e.to_string()))?;
        Ok(())
    }

    /// Loads a DOM Fingerprint.
    pub fn load(&self, url: &str, selector: &str) -> Result<Option<DomFingerprint>> {
        let key = format!("{}:{}", url, selector);
        let bytes_opt = self.db.get(key.as_bytes())
            .map_err(|e| CrawlingoError::FingerprintStoreError(e.to_string()))?;
        
        match bytes_opt {
            Some(ivec) => {
                let fingerprint = bincode::deserialize(&ivec)
                    .map_err(|e| CrawlingoError::BincodeError(e))?;
                Ok(Some(fingerprint))
            }
            None => Ok(None)
        }
    }

    /// Deletes a cached fingerprint record.
    pub fn delete(&self, url: &str, selector: &str) -> Result<()> {
        let key = format!("{}:{}", url, selector);
        self.db.remove(key.as_bytes())
            .map_err(|e| CrawlingoError::FingerprintStoreError(e.to_string()))?;
        self.db.flush()
            .map_err(|e| CrawlingoError::FingerprintStoreError(e.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_store_and_load_fingerprint() {
        let temp_dir = tempfile::tempdir().unwrap();
        let store = FingerprintStore::open(temp_dir.path()).unwrap();
        
        let fp = DomFingerprint {
            tag: "div".to_string(),
            text: "test".to_string(),
            html_snippet: "<div>".to_string(),
            depth: 3,
            sibling_index: 2,
            parent_tag: "body".to_string(),
            parent_class: "".to_string(),
            parent_id: "".to_string(),
            attributes: HashMap::new(),
            class_list: vec![],
            id: None,
            ancestor_path: vec![],
            hash: 42,
            captured_at: chrono::Utc::now(),
            url: "http://example.com".to_string(),
            selector_used: ".test".to_string(),
            similarity_score: 1.0,
        };

        store.store("http://example.com", ".test", &fp).unwrap();
        
        let loaded = store.load("http://example.com", ".test").unwrap().unwrap();
        assert_eq!(loaded.tag, "div");
        assert_eq!(loaded.text, "test");
        assert_eq!(loaded.hash, 42);

        store.delete("http://example.com", ".test").unwrap();
        assert!(store.load("http://example.com", ".test").unwrap().is_none());
    }
}
