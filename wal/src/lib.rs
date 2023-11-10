use anyhow::Result;
use mlg::{Bytes, FileLog, Log};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::{Mutex, RwLock};

type Key = Bytes;
type Value = Bytes;

const PAGINATION_SIZE: usize = 128;

#[derive(Debug, Serialize, Deserialize)]
enum Operation {
    Put(Key, Value),
    Get(Key),
    Delete(Key),
}

struct Storage {
    data: RwLock<HashMap<Key, Value>>,
    log: Mutex<FileLog>,
}

impl Storage {
    pub async fn new(log: FileLog) -> Result<Self> {
        let mut data = HashMap::new();

        let mut offset = 0;
        let mut done = false;

        while !done {
            // Read a batch of records from the log
            let (records, next_offset) = log.batch_read(offset, PAGINATION_SIZE).await?;

            // Check if we've reached the end of the log
            if records.is_empty() {
                done = true;
                break;
            }

            // Process the records
            for record in records {
                // Deserialize the record
                let operation: Operation = bincode::deserialize(&record)?;

                // Apply the operation to the data
                match operation {
                    Operation::Put(key, value) => {
                        data.insert(key, value);
                    }
                    Operation::Get(key) => {
                        data.get(&key);
                    }
                    Operation::Delete(key) => {
                        data.remove(&key);
                    }
                }
            }

            // Advance the offset
            offset = next_offset;
        }

        Ok(Storage {
            data: RwLock::new(data),
            log: Mutex::new(log),
        })
    }

    pub async fn put(&self, key: Key, value: Value) -> Result<()> {
        let operation = Operation::Put(key.clone(), value.clone());
        let bytes = bincode::serialize(&operation)?;
        {
            // Write to the log
            let log = self.log.lock().await;
            log.append(bytes).await?;
        }
        {
            // Write to the data
            let mut data = self.data.write().await;
            data.insert(key, value);
        }
        Ok(())
    }

    pub async fn get(&self, key: Key) -> Result<Option<Value>> {
        let data = self.data.read().await;
        Ok(data.get(&key).cloned())
    }

    pub async fn delete(&self, key: Key) -> Result<()> {
        let operation = Operation::Delete(key.clone());
        let bytes = bincode::serialize(&operation)?;
        {
            // Write to the log
            let log = self.log.lock().await;
            log.append(bytes).await?;
        }
        {
            // Write to the data
            let mut data = self.data.write().await;
            data.remove(&key);
        }
        Ok(())
    }
}