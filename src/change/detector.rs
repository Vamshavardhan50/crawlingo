use std::collections::{HashMap, HashSet};
use rayon::prelude::*;
use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum ChangeType {
    ContentChange,
    PriceChange { old_price: f64, new_price: f64, diff_pct: f64 },
    StockChange { in_stock: bool },
    ElementAdded,
    ElementRemoved,
    LayoutChange,
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
    let clean: String = val.chars()
        .filter(|c| c.is_digit(10) || *c == '.' || *c == '-')
        .collect();
    clean.parse::<f64>().ok()
}

// Helper to detect stock status
fn is_stock_status(val: &str) -> bool {
    let l = val.to_lowercase();
    l.contains("in stock") || l.contains("out of stock") || l.contains("sold out") || l.contains("available")
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

    fields_vec.into_par_iter()
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
                        let change_type = if let (Some(old_price), Some(new_price)) = (parse_price(old_val), parse_price(new_val)) {
                            // Check if it looks like a price change
                            let diff_pct = if old_price != 0.0 {
                                ((new_price - old_price) / old_price) * 100.0
                            } else {
                                0.0
                            };
                            ChangeType::PriceChange { old_price, new_price, diff_pct }
                        } else if is_stock_status(old_val) || is_stock_status(new_val) {
                            let in_stock = new_val.to_lowercase().contains("in stock") || new_val.to_lowercase().contains("available");
                            ChangeType::StockChange { in_stock }
                        } else {
                            ChangeType::ContentChange
                        };

                        let diff = format!("- {}\n+ {}", old_val, new_val);

                        Some(ChangeEvent {
                            url: url.to_string(),
                            field: field.clone(),
                            change_type,
                            old_value: old_val.clone(),
                            new_value: new_val.clone(),
                            diff,
                            detected_at: Utc::now(),
                            similarity_score: 1.0,
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
            ChangeType::LayoutChange => "layout",
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
