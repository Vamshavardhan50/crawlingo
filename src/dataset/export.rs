use crate::error::{CrawlingoError, Result};
use arrow::array::{ArrayRef, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::arrow::arrow_writer::ArrowWriter;
use parquet::file::properties::WriterProperties;
use std::collections::HashMap;
use std::sync::Arc;

/// Exports key-value fields to a Parquet format file at the specified path.
pub async fn write_parquet(path: &str, fields: &HashMap<String, String>) -> Result<()> {
    if fields.is_empty() {
        return Err(CrawlingoError::ExportError(
            "Cannot export empty dataset".to_string(),
        ));
    }

    // 1. Map fields to Arrow schema and arrays
    let mut arrow_fields = Vec::new();
    let mut arrays: Vec<ArrayRef> = Vec::new();

    for (name, val) in fields {
        arrow_fields.push(Field::new(name, DataType::Utf8, false));
        let array = StringArray::from(vec![val.as_str()]);
        arrays.push(Arc::new(array));
    }

    let schema = Arc::new(Schema::new(arrow_fields));

    // 2. Build RecordBatch
    let batch = RecordBatch::try_new(schema, arrays).map_err(|e| CrawlingoError::ArrowError(e))?;

    // 3. Write Parquet File
    // std::fs::File is blocked using tokio::task::block_in_place or run inside spawn_blocking
    let path_str = path.to_string();
    tokio::task::spawn_blocking(move || {
        let file = std::fs::File::create(&path_str)?;
        let props = WriterProperties::builder().build();
        let mut writer = ArrowWriter::try_new(file, batch.schema(), Some(props))?;
        writer.write(&batch)?;
        writer.close()?;
        Ok::<(), CrawlingoError>(())
    })
    .await
    .map_err(|e| CrawlingoError::ExportError(format!("Task execution panicked: {}", e)))?
    .map_err(|e| CrawlingoError::ExportError(e.to_string()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_parquet_export() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.parquet");
        let path_str = file_path.to_str().unwrap();

        let mut fields = HashMap::new();
        fields.insert("title".to_string(), "Book One".to_string());
        fields.insert("price".to_string(), "$19.99".to_string());

        let res = write_parquet(path_str, &fields).await;
        assert!(res.is_ok());
        assert!(file_path.exists());
    }
}
