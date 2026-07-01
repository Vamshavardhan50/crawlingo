use crate::error::Result;
use std::collections::HashMap;

/// An async streaming buffer for dataset records.
///
/// Uses a tokio mpsc channel to stream extracted records from workers
/// to a background writer, enabling constant-memory scaling for large crawls.
pub struct DatasetStream {
    sender: tokio::sync::mpsc::UnboundedSender<Result<HashMap<String, String>>>,
    receiver: tokio::sync::mpsc::UnboundedReceiver<Result<HashMap<String, String>>>,
    count: usize,
}

impl DatasetStream {
    /// Create a new unbounded streaming buffer.
    pub fn new() -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        Self {
            sender: tx,
            receiver: rx,
            count: 0,
        }
    }

    /// Push a record into the stream.
    pub fn push(&self, record: Result<HashMap<String, String>>) {
        let _ = self.sender.send(record);
    }

    /// Try to receive the next record (non-blocking).
    pub fn try_recv(&mut self) -> Option<Result<HashMap<String, String>>> {
        match self.receiver.try_recv() {
            Ok(record) => {
                if record.is_ok() {
                    self.count += 1;
                }
                Some(record)
            }
            Err(_) => None,
        }
    }

    /// Receive the next record asynchronously.
    pub async fn recv(&mut self) -> Option<Result<HashMap<String, String>>> {
        let record = self.receiver.recv().await;
        if let Some(Ok(_)) = record.as_ref() {
            self.count += 1;
        }
        record
    }

    /// Number of successfully received records.
    pub fn count(&self) -> usize {
        self.count
    }

    /// Drain remaining records into a vector (blocking; for small datasets).
    pub fn drain(&mut self) -> Vec<Result<HashMap<String, String>>> {
        let mut records = Vec::new();
        while let Some(record) = self.try_recv() {
            records.push(record);
        }
        records
    }

    /// Consume the stream and write all records to a CSV file.
    pub async fn write_csv(mut self, path: &str) -> Result<usize> {
        let mut writer = csv::Writer::from_path(path)
            .map_err(|e| crate::error::CrawlingoError::DatasetError(e.to_string()))?;
        let mut header_written = false;
        let mut total = 0;

        while let Some(record) = self.recv().await {
            match record {
                Ok(fields) => {
                    if !header_written {
                        let keys: Vec<&str> = fields.keys().map(|k| k.as_str()).collect();
                        writer.write_record(&keys)
                            .map_err(|e| crate::error::CrawlingoError::DatasetError(e.to_string()))?;
                        header_written = true;
                    }
                    let values: Vec<&str> = fields.values().map(|v| v.as_str()).collect();
                    writer.write_record(&values)
                        .map_err(|e| crate::error::CrawlingoError::DatasetError(e.to_string()))?;
                    total += 1;
                }
                Err(e) => {
                    tracing::warn!("Skipping failed record in stream: {e}");
                }
            }
        }

        writer.flush()
            .map_err(|e| crate::error::CrawlingoError::DatasetError(e.to_string()))?;
        Ok(total)
    }
}

impl Default for DatasetStream {
    fn default() -> Self {
        Self::new()
    }
}
