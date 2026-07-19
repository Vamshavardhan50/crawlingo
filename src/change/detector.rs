use chrono::{DateTime, Utc};
use rayon::prelude::*;
use serde::Serialize;
use std::collections::{HashMap, HashSet};

#[non_exhaustive]
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum ChangeType {
    ContentChange,
    PriceChange {
        old_price: f64,
        new_price: f64,
        diff_pct: f64,
    },
    StockChange {
        in_stock: bool,
    },
    ElementAdded,
    ElementRemoved,
}

/// A structured change event indicating modifications on a parsed field.
#[derive(Debug, Clone, Serialize)]
pub struct ChangeEvent {
    pub url: String,
    pub field: String,
    pub change_type: ChangeType,
    pub old_value: String,
    pub new_value: String,
    pub diff: String,
    pub detected_at: DateTime<Utc>,
    pub similarity_score: f64,
}

// Helper to strip currency and parse floats
fn parse_price(val: &str) -> Option<f64> {
    crate::util::price::parse_price_string(val).and_then(|s| s.parse::<f64>().ok())
}

// Helper to detect stock status
fn is_stock_status(val: &str) -> bool {
    let l = val.to_lowercase();
    l.contains("in stock")
        || l.contains("out of stock")
        || l.contains("sold out")
        || l.contains("available")
}

/// Detects changes between old and new dataset maps in parallel.
pub fn detect_changes(
    url: &str,
    old_data: &HashMap<String, String>,
    new_data: &HashMap<String, String>,
) -> Vec<ChangeEvent> {
    let mut all_fields = HashSet::new();
    for k in old_data.keys() {
        all_fields.insert(k.clone());
    }
    for k in new_data.keys() {
        all_fields.insert(k.clone());
    }

    let fields_vec: Vec<String> = all_fields.into_iter().collect();

    fields_vec
        .into_par_iter()
        .filter_map(|field| {
            let old_opt = old_data.get(&field);
            let new_opt = new_data.get(&field);

            match (old_opt, new_opt) {
                (None, Some(new_val)) => Some(ChangeEvent {
                    url: url.to_string(),
                    field: field.clone(),
                    change_type: ChangeType::ElementAdded,
                    old_value: String::new(),
                    new_value: new_val.clone(),
                    diff: format!("+ {}", new_val),
                    detected_at: Utc::now(),
                    similarity_score: 1.0,
                }),
                (Some(old_val), None) => Some(ChangeEvent {
                    url: url.to_string(),
                    field: field.clone(),
                    change_type: ChangeType::ElementRemoved,
                    old_value: old_val.clone(),
                    new_value: String::new(),
                    diff: format!("- {}", old_val),
                    detected_at: Utc::now(),
                    similarity_score: 1.0,
                }),
                (Some(old_val), Some(new_val)) => {
                    if old_val == new_val {
                        None // No change
                    } else {
                        // Classify change type
                        let change_type = if let (Some(old_price), Some(new_price)) =
                            (parse_price(old_val), parse_price(new_val))
                        {
                            // Check if it looks like a price change
                            let diff_pct = if old_price != 0.0 {
                                ((new_price - old_price) / old_price) * 100.0
                            } else {
                                0.0
                            };
                            ChangeType::PriceChange {
                                old_price,
                                new_price,
                                diff_pct,
                            }
                        } else if is_stock_status(old_val) || is_stock_status(new_val) {
                            let in_stock = new_val.to_lowercase().contains("in stock")
                                || new_val.to_lowercase().contains("available");
                            ChangeType::StockChange { in_stock }
                        } else {
                            ChangeType::ContentChange
                        };

                        let diff = format!("- {}\n+ {}", old_val, new_val);
                        let similarity_score = strsim::jaro_winkler(old_val, new_val);

                        Some(ChangeEvent {
                            url: url.to_string(),
                            field: field.clone(),
                            change_type,
                            old_value: old_val.clone(),
                            new_value: new_val.clone(),
                            diff,
                            detected_at: Utc::now(),
                            similarity_score,
                        })
                    }
                }
                (None, None) => None,
            }
        })
        .collect()
}

// PyO3 Bindings
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pyclass(name = "ChangeEvent")]
#[derive(Clone)]
pub struct PyChangeEvent {
    #[pyo3(get)]
    pub url: String,
    #[pyo3(get)]
    pub field: String,
    #[pyo3(get)]
    pub change_type: String, // "content", "price", "stock", "added", "removed", "layout"
    #[pyo3(get)]
    pub old_value: String,
    #[pyo3(get)]
    pub new_value: String,
    #[pyo3(get)]
    pub diff: String,
    #[pyo3(get)]
    pub detected_at: String,
    #[pyo3(get)]
    pub similarity_score: f64,
}

#[cfg(feature = "python")]
impl From<ChangeEvent> for PyChangeEvent {
    fn from(evt: ChangeEvent) -> Self {
        let type_str = match evt.change_type {
            ChangeType::ContentChange => "content",
            ChangeType::PriceChange { .. } => "price",
            ChangeType::StockChange { .. } => "stock",
            ChangeType::ElementAdded => "added",
            ChangeType::ElementRemoved => "removed",
        };

        Self {
            url: evt.url,
            field: evt.field,
            change_type: type_str.to_string(),
            old_value: evt.old_value,
            new_value: evt.new_value,
            diff: evt.diff,
            detected_at: evt.detected_at.to_rfc3339(),
            similarity_score: evt.similarity_score,
        }
    }
}

#[cfg(feature = "python")]
#[pymethods]
impl PyChangeEvent {
    fn __repr__(&self) -> String {
        format!(
            "ChangeEvent(field='{}', type='{}', old='{}', new='{}')",
            self.field, self.change_type, self.old_value, self.new_value
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_similarity_score_jaro_winkler() {
        let mut old = HashMap::new();
        old.insert("title".to_string(), "Awesome Product".to_string());

        let mut new = HashMap::new();
        new.insert("title".to_string(), "Awesom Product!".to_string());

        let changes = detect_changes("https://example.com", &old, &new);
        assert_eq!(changes.len(), 1);
        let score = changes[0].similarity_score;
        assert!(
            score > 0.8 && score < 1.0,
            "Score should be Jaro-Winkler, got {}",
            score
        );

        let mut new_diff = HashMap::new();
        new_diff.insert(
            "title".to_string(),
            "Totally Different Title String".to_string(),
        );
        let changes_diff = detect_changes("https://example.com", &old, &new_diff);
        assert_eq!(changes_diff.len(), 1);
        let score_diff = changes_diff[0].similarity_score;
        assert!(
            score_diff < 0.6,
            "Dissimilar score should be low, got {}",
            score_diff
        );
    }

    #[test]
    fn test_change_detection_variants() {
        // 1. ContentChange
        let mut old = HashMap::new();
        old.insert("title".to_string(), "Old Title".to_string());
        let mut new = HashMap::new();
        new.insert("title".to_string(), "New Title".to_string());
        let changes = detect_changes("https://example.com", &old, &new);
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].field, "title");
        assert!(matches!(changes[0].change_type, ChangeType::ContentChange));

        // 2. PriceChange
        let mut old_price = HashMap::new();
        old_price.insert("price".to_string(), "$100.00".to_string());
        let mut new_price = HashMap::new();
        new_price.insert("price".to_string(), "$120.00".to_string());
        let changes_price = detect_changes("https://example.com", &old_price, &new_price);
        assert_eq!(changes_price.len(), 1);
        if let ChangeType::PriceChange {
            old_price,
            new_price,
            diff_pct,
        } = changes_price[0].change_type
        {
            assert_eq!(old_price, 100.0);
            assert_eq!(new_price, 120.0);
            assert!((diff_pct - 20.0).abs() < 1e-5);
        } else {
            panic!("Expected PriceChange");
        }

        // 3. StockChange
        let mut old_stock = HashMap::new();
        old_stock.insert("stock".to_string(), "In Stock".to_string());
        let mut new_stock = HashMap::new();
        new_stock.insert("stock".to_string(), "Out of Stock".to_string());
        let changes_stock = detect_changes("https://example.com", &old_stock, &new_stock);
        assert_eq!(changes_stock.len(), 1);
        if let ChangeType::StockChange { in_stock } = changes_stock[0].change_type {
            assert!(!in_stock);
        } else {
            panic!("Expected StockChange");
        }

        // 4. ElementAdded (None -> Some)
        let old_add = HashMap::new();
        let mut new_add = HashMap::new();
        new_add.insert("extra".to_string(), "new element".to_string());
        let changes_add = detect_changes("https://example.com", &old_add, &new_add);
        assert_eq!(changes_add.len(), 1);
        assert_eq!(changes_add[0].field, "extra");
        assert!(matches!(
            changes_add[0].change_type,
            ChangeType::ElementAdded
        ));

        // 5. ElementRemoved (Some -> None)
        let mut old_rem = HashMap::new();
        old_rem.insert("extra".to_string(), "old element".to_string());
        let new_rem = HashMap::new();
        let changes_rem = detect_changes("https://example.com", &old_rem, &new_rem);
        assert_eq!(changes_rem.len(), 1);
        assert_eq!(changes_rem[0].field, "extra");
        assert!(matches!(
            changes_rem[0].change_type,
            ChangeType::ElementRemoved
        ));

        // 6. NoChange
        let mut old_no = HashMap::new();
        old_no.insert("field".to_string(), "value".to_string());
        let mut new_no = HashMap::new();
        new_no.insert("field".to_string(), "value".to_string());
        let changes_no = detect_changes("https://example.com", &old_no, &new_no);
        assert!(changes_no.is_empty());
    }
}
