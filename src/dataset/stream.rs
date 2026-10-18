use crate::error::Result;
use std::collections::HashMap;

/// An async streaming buffer for dataset records.
///
/// Uses a tokio mpsc channel to stream extracted records from workers
/// to a consumer with bounded channel capacity. Record sizes and export buffers
/// also contribute to memory usage.
pub struct DatasetStream {
    /// `None` once [`DatasetStream::detach_sender`] has been called. A drain loop (`recv()`,
    /// `write_csv`, `write_parquet`) can only ever observe the channel as closed once every
    /// sender — both this field, if still `Some`, and any [`DatasetStreamHandle`] clone handed to
    /// a producer — has been dropped. Without a way to give up this field's own sender, a caller
    /// holding both a `DatasetStream` and the task producing into it (e.g. via
    /// [`crate::dataset::builder::Dataset::build_many_streamed`]) could never see the stream end.
    sender: Option<tokio::sync::mpsc::Sender<Result<HashMap<String, String>>>>,
    receiver: tokio::sync::mpsc::Receiver<Result<HashMap<String, String>>>,
    count: usize,
}

impl DatasetStream {
    /// Create a bounded streaming buffer with capacity for 100 records.
    pub fn new() -> Self {
        Self::with_capacity(100)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let (tx, rx) = tokio::sync::mpsc::channel(capacity.max(1));
        Self {
            sender: Some(tx),
            receiver: rx,
            count: 0,
        }
    }

    /// Try to push a record. Returns an error if full, closed or detached.
    /// Use [`DatasetStreamHandle::send`] to wait for available capacity.
    pub fn push(&self, record: Result<HashMap<String, String>>) -> Result<()> {
        self.sender
            .as_ref()
            .ok_or_else(|| {
                crate::error::CrawlingoError::DatasetError("stream sender detached".into())
            })?
            .try_send(record)
            .map_err(|e| crate::error::CrawlingoError::DatasetError(e.to_string()))
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

    /// Returns a cloneable handle that other tasks can use to push records into this stream.
    ///
    /// Necessary because a `DatasetStream` bundles both ends of the channel — a producer running
    /// in a separate spawned task (e.g. [`crate::dataset::builder::Dataset::build_many_streamed`])
    /// needs a way to send records in while the caller retains the `DatasetStream` itself to
    /// consume them. Must be called before [`DatasetStream::detach_sender`].
    pub fn handle(&self) -> DatasetStreamHandle {
        DatasetStreamHandle {
            sender: self
                .sender
                .clone()
                .expect("DatasetStream::handle called after detach_sender"),
        }
    }

    /// Drops this stream's own internal sender.
    ///
    /// Once every producer has its own [`DatasetStreamHandle`] (from [`DatasetStream::handle`]),
    /// call this so a `recv()`-based drain loop — including one the caller writes directly against
    /// the returned `DatasetStream`, not just `write_csv`/`write_parquet` — can observe the
    /// channel as closed as soon as those handles are all dropped, instead of waiting forever on a
    /// sender this struct would otherwise keep alive for its entire lifetime.
    pub fn detach_sender(&mut self) {
        self.sender = None;
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
    ///
    /// `self` bundles both channel ends (see [`DatasetStream::handle`]), so before draining we
    /// must drop `self`'s own sender — otherwise the channel could never report "closed" and this
    /// loop would wait forever even after every producer handle has finished.
    pub async fn write_csv(self, path: &str) -> Result<usize> {
        let mut writer = csv::Writer::from_path(path)
            .map_err(|e| crate::error::CrawlingoError::DatasetError(e.to_string()))?;
        let mut keys: Option<Vec<String>> = None;
        let mut total = 0;

        let DatasetStream {
            sender,
            mut receiver,
            ..
        } = self;
        drop(sender);

        while let Some(record) = receiver.recv().await {
            match record {
                Ok(fields) => {
                    if keys.is_none() {
                        let mut columns: Vec<String> = fields.keys().cloned().collect();
                        columns.sort();
                        writer.write_record(&columns).map_err(|e| {
                            crate::error::CrawlingoError::DatasetError(e.to_string())
                        })?;
                        keys = Some(columns);
                    }
                    let columns = keys.as_ref().unwrap();
                    if fields.keys().any(|key| !columns.contains(key)) {
                        return Err(crate::error::CrawlingoError::ExportError(
                            "CSV record introduces undeclared columns".into(),
                        ));
                    }
                    let values: Vec<&str> = columns
                        .iter()
                        .map(|key| fields.get(key).map(String::as_str).unwrap_or(""))
                        .collect();
                    writer
                        .write_record(&values)
                        .map_err(|e| crate::error::CrawlingoError::DatasetError(e.to_string()))?;
                    total += 1;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        writer
            .flush()
            .map_err(|e| crate::error::CrawlingoError::DatasetError(e.to_string()))?;
        Ok(total)
    }

    /// Consume the stream and write all records to a Parquet file.
    ///
    /// Forwards successfully-received records onto a fresh channel consumed by
    /// [`crate::dataset::export::write_parquet_stream`], propagating failed records
    /// exactly like [`DatasetStream::write_csv`]. See that method's doc comment for why `self`'s
    /// own sender must be dropped before draining.
    pub async fn write_parquet(self, path: &str) -> Result<usize> {
        self.write_parquet_with_batch_size(path, 1000).await
    }

    /// Consume the stream and write all records to a Parquet file using a custom batch size.
    pub async fn write_parquet_with_batch_size(
        self,
        path: &str,
        batch_size: usize,
    ) -> Result<usize> {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        let DatasetStream {
            sender,
            mut receiver,
            ..
        } = self;
        drop(sender);
        let forward = async move {
            let mut total = 0;
            while let Some(record) = receiver.recv().await {
                tx.send(record?).await.map_err(|_| {
                    crate::error::CrawlingoError::ExportError("Parquet writer closed".into())
                })?;
                total += 1;
            }
            Ok::<usize, crate::error::CrawlingoError>(total)
        };
        // Poll both ends concurrently. Sending before starting the writer deadlocked at capacity.
        let (forwarded, written) = tokio::join!(
            forward,
            crate::dataset::export::write_parquet_stream(path, rx, Some(batch_size))
        );
        written?;
        forwarded
    }
}

/// A cloneable handle for pushing records into a [`DatasetStream`] from another task. See
/// [`DatasetStream::handle`].
#[derive(Clone)]
pub struct DatasetStreamHandle {
    sender: tokio::sync::mpsc::Sender<Result<HashMap<String, String>>>,
}

impl DatasetStreamHandle {
    /// Pushes a record into the associated stream.
    pub fn push(&self, record: Result<HashMap<String, String>>) -> Result<()> {
        self.sender
            .try_send(record)
            .map_err(|e| crate::error::CrawlingoError::DatasetError(e.to_string()))
    }

    /// Waits for consumer capacity; producers stop when the receiver is dropped.
    pub async fn send(&self, record: Result<HashMap<String, String>>) -> Result<()> {
        self.sender
            .send(record)
            .await
            .map_err(|e| crate::error::CrawlingoError::DatasetError(e.to_string()))
    }

    pub fn is_closed(&self) -> bool {
        self.sender.is_closed()
    }
}

impl Default for DatasetStream {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn handle_push_is_observed_via_recv() {
        let mut stream = DatasetStream::new();
        let handle = stream.handle();
        handle.push(Ok(HashMap::new())).unwrap();
        assert!(stream.recv().await.is_some());
    }

    #[tokio::test]
    async fn write_csv_terminates_once_every_handle_is_dropped() {
        // Regression test: `DatasetStream` bundles both channel ends, so `write_csv`/
        // `write_parquet` must drop `self`'s own sender before draining — otherwise the loop
        // would wait forever for a "channel closed" signal that could never arrive.
        let stream = DatasetStream::new();
        let handle = stream.handle();

        let mut a = HashMap::new();
        a.insert("title".to_string(), "Book One".to_string());
        handle.push(Ok(a)).unwrap();
        drop(handle);

        let dir = tempdir().unwrap();
        let path = dir.path().join("stream.csv");
        let total = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            stream.write_csv(path.to_str().unwrap()),
        )
        .await
        .expect("write_csv must terminate once all producer handles are dropped")
        .unwrap();

        assert_eq!(total, 1);
        assert!(path.exists());
    }

    #[tokio::test]
    async fn write_parquet_streams_pushed_records() {
        let stream = DatasetStream::new();
        let handle = stream.handle();

        let mut a = HashMap::new();
        a.insert("title".to_string(), "Book One".to_string());
        handle.push(Ok(a)).unwrap();

        let mut b = HashMap::new();
        b.insert("title".to_string(), "Book Two".to_string());
        handle.push(Ok(b)).unwrap();

        drop(handle);

        let dir = tempdir().unwrap();
        let path = dir.path().join("stream.parquet");
        let total = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            stream.write_parquet(path.to_str().unwrap()),
        )
        .await
        .expect("write_parquet must terminate once all producer handles are dropped")
        .unwrap();

        assert_eq!(total, 2);
        assert!(path.exists());
    }
}
