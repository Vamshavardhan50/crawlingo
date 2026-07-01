use crate::error::{CrawlingoError, Result};
use std::collections::HashMap;

/// Supported field data types for schema validation.
#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    String,
    Integer,
    Float,
    Boolean,
}

impl FieldType {
    /// Parse a string value into the target type and return a display string.
    pub fn validate(&self, value: &str, field_name: &str) -> Result<String> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Ok(String::new());
        }
        match self {
            FieldType::String => Ok(trimmed.to_string()),
            FieldType::Integer => {
                let cleaned: String = trimmed.chars().filter(|c| c.is_ascii_digit() || *c == '-').collect();
                cleaned
                    .parse::<i64>()
                    .map(|_| cleaned)
                    .map_err(|_| CrawlingoError::DatasetError(format!(
                        "Field '{}': value '{}' is not a valid integer", field_name, trimmed
                    )))
            }
            FieldType::Float => {
                let cleaned: String = trimmed.chars().filter(|c| c.is_ascii_digit() || *c == '.' || *c == '-').collect();
                // Handle duplicate dots
                let dot_count = cleaned.matches('.').count();
                let normalized = if dot_count > 1 {
                    let mut parts: Vec<&str> = cleaned.split('.').collect();
                    let decimal = parts.pop().unwrap_or("0");
                    let integer = parts.join("");
                    format!("{}.{}", integer, decimal)
                } else {
                    cleaned
                };
                normalized
                    .parse::<f64>()
                    .map(|_| normalized)
                    .map_err(|_| CrawlingoError::DatasetError(format!(
                        "Field '{}': value '{}' is not a valid float", field_name, trimmed
                    )))
            }
            FieldType::Boolean => {
                match trimmed.to_lowercase().as_str() {
                    "true" | "1" | "yes" => Ok("true".to_string()),
                    "false" | "0" | "no" => Ok("false".to_string()),
                    _ => Err(CrawlingoError::DatasetError(format!(
                        "Field '{}': value '{}' is not a valid boolean", field_name, trimmed
                    ))),
                }
            }
        }
    }
}

/// A field constraint in a dataset schema.
#[derive(Debug, Clone)]
pub struct FieldConstraint {
    pub name: String,
    pub field_type: FieldType,
    pub required: bool,
}

impl FieldConstraint {
    pub fn new(name: &str, field_type: FieldType, required: bool) -> Self {
        Self {
            name: name.to_string(),
            field_type,
            required,
        }
    }
}

/// A dataset schema defining expected fields and their constraints.
#[derive(Debug, Clone)]
pub struct DatasetSchema {
    pub fields: Vec<FieldConstraint>,
}

impl DatasetSchema {
    pub fn new(fields: Vec<FieldConstraint>) -> Self {
        Self { fields }
    }

    /// Validate a field map against this schema.
    /// Returns Ok(validated_map) where all values are type-converted,
    /// or Err with the first validation failure.
    pub fn validate(&self, record: &HashMap<String, String>) -> Result<HashMap<String, String>> {
        let mut validated = HashMap::new();

        for constraint in &self.fields {
            let raw_value = record.get(&constraint.name).map(|s| s.as_str()).unwrap_or("");

            if constraint.required && raw_value.trim().is_empty() {
                return Err(CrawlingoError::DatasetError(format!(
                    "Required field '{}' is missing or empty", constraint.name
                )));
            }

            let validated_value = constraint.field_type.validate(raw_value, &constraint.name)?;
            validated.insert(constraint.name.clone(), validated_value);
        }

        Ok(validated)
    }
}

impl Default for DatasetSchema {
    fn default() -> Self {
        Self { fields: Vec::new() }
    }
}
