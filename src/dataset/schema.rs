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
                let cleaned: String = trimmed
                    .chars()
                    .filter(|c| c.is_ascii_digit() || *c == '-')
                    .collect();
                cleaned.parse::<i64>().map(|_| cleaned).map_err(|_| {
                    CrawlingoError::DatasetError(format!(
                        "Field '{}': value '{}' is not a valid integer",
                        field_name, trimmed
                    ))
                })
            }
            FieldType::Float => crate::util::price::parse_price_string(trimmed).ok_or_else(|| {
                CrawlingoError::DatasetError(format!(
                    "Field '{}': value '{}' is not a valid float",
                    field_name, trimmed
                ))
            }),
            FieldType::Boolean => match trimmed.to_lowercase().as_str() {
                "true" | "1" | "yes" => Ok("true".to_string()),
                "false" | "0" | "no" => Ok("false".to_string()),
                _ => Err(CrawlingoError::DatasetError(format!(
                    "Field '{}': value '{}' is not a valid boolean",
                    field_name, trimmed
                ))),
            },
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
#[derive(Debug, Clone, Default)]
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
            let raw_value = record
                .get(&constraint.name)
                .map(|s| s.as_str())
                .unwrap_or("");

            if constraint.required && raw_value.trim().is_empty() {
                return Err(CrawlingoError::DatasetError(format!(
                    "Required field '{}' is missing or empty",
                    constraint.name
                )));
            }

            let validated_value = constraint
                .field_type
                .validate(raw_value, &constraint.name)?;
            validated.insert(constraint.name.clone(), validated_value);
        }

        Ok(validated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_type_string() {
        let f = FieldType::String;
        assert_eq!(f.validate("  hello  ", "field").unwrap(), "hello");
        assert_eq!(f.validate("", "field").unwrap(), "");
    }

    #[test]
    fn test_field_type_integer() {
        let f = FieldType::Integer;
        assert_eq!(f.validate("123", "field").unwrap(), "123");
        assert_eq!(f.validate(" -456 ", "field").unwrap(), "-456");
        assert!(f.validate("abc", "field").is_err());
    }

    #[test]
    fn test_field_type_float() {
        let f = FieldType::Float;
        assert_eq!(f.validate("12.34", "field").unwrap(), "12.34");
        assert_eq!(f.validate(" -1.234.56 ", "field").unwrap(), "-1234.56");
        assert!(f.validate("abc", "field").is_err());
    }

    #[test]
    fn test_field_type_boolean() {
        let f = FieldType::Boolean;
        // Truthy
        assert_eq!(f.validate("true", "field").unwrap(), "true");
        assert_eq!(f.validate("1", "field").unwrap(), "true");
        assert_eq!(f.validate("yes", "field").unwrap(), "true");
        // Falsy
        assert_eq!(f.validate("false", "field").unwrap(), "false");
        assert_eq!(f.validate("0", "field").unwrap(), "false");
        assert_eq!(f.validate("no", "field").unwrap(), "false");
        // Invalid
        assert!(f.validate("maybe", "field").is_err());
    }

    #[test]
    fn test_dataset_schema_validate() {
        let schema = DatasetSchema::new(vec![
            FieldConstraint::new("title", FieldType::String, true),
            FieldConstraint::new("price", FieldType::Float, false),
            FieldConstraint::new("qty", FieldType::Integer, true),
        ]);

        // 1. Valid record
        let mut rec = HashMap::new();
        rec.insert("title".to_string(), "Book".to_string());
        rec.insert("price".to_string(), "$19.99".to_string());
        rec.insert("qty".to_string(), " 2 ".to_string());
        let val = schema.validate(&rec).unwrap();
        assert_eq!(val.get("title").unwrap(), "Book");
        assert_eq!(val.get("price").unwrap(), "19.99");
        assert_eq!(val.get("qty").unwrap(), "2");

        // 2. Required field missing
        let mut rec_missing = HashMap::new();
        rec_missing.insert("price".to_string(), "$19.99".to_string());
        rec_missing.insert("qty".to_string(), "2".to_string());
        assert!(schema.validate(&rec_missing).is_err());

        // 3. Optional field empty
        let mut rec_opt_empty = HashMap::new();
        rec_opt_empty.insert("title".to_string(), "Book".to_string());
        rec_opt_empty.insert("price".to_string(), "".to_string());
        rec_opt_empty.insert("qty".to_string(), "2".to_string());
        let val_opt = schema.validate(&rec_opt_empty).unwrap();
        assert_eq!(val_opt.get("title").unwrap(), "Book");
        assert_eq!(val_opt.get("price").unwrap(), "");
    }
}

// PyO3 Bindings
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pyclass(name = "FieldType", eq, eq_int)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PyFieldType {
    String,
    Integer,
    Float,
    Boolean,
}

#[cfg(feature = "python")]
impl From<PyFieldType> for FieldType {
    fn from(ft: PyFieldType) -> Self {
        match ft {
            PyFieldType::String => FieldType::String,
            PyFieldType::Integer => FieldType::Integer,
            PyFieldType::Float => FieldType::Float,
            PyFieldType::Boolean => FieldType::Boolean,
        }
    }
}

#[cfg(feature = "python")]
#[pyclass(name = "FieldConstraint")]
#[derive(Debug, Clone)]
pub struct PyFieldConstraint {
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub field_type: PyFieldType,
    #[pyo3(get)]
    pub required: bool,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyFieldConstraint {
    #[new]
    pub fn new(name: String, field_type: PyFieldType, required: bool) -> Self {
        Self {
            name,
            field_type,
            required,
        }
    }
}

#[cfg(feature = "python")]
#[pyclass(name = "DatasetSchema")]
#[derive(Debug, Clone, Default)]
pub struct PyDatasetSchema {
    pub inner: DatasetSchema,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyDatasetSchema {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: DatasetSchema::default(),
        }
    }

    pub fn add_field(&mut self, name: &str, field_type: PyFieldType, required: bool) {
        self.inner.fields.push(FieldConstraint {
            name: name.to_string(),
            field_type: field_type.into(),
            required,
        });
    }

    pub fn validate(&self, record: HashMap<String, String>) -> PyResult<HashMap<String, String>> {
        self.inner
            .validate(&record)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }
}
