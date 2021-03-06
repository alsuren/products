use super::models::{StorageClientError, StorageFile};
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait StorageClient {
    async fn add_file(
        &self,
        file_data: &[u8],
        metadata_ref: HashMap<&str, &str>,
    ) -> Result<StorageFile, StorageClientError>;
    async fn get_file(
        &self,
        storage_file_identifier: StorageFile,
    ) -> Result<Vec<u8>, StorageClientError>;
}
