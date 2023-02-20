#![feature(async_fn_in_trait)]
use crate::types::file_object::FileObject;
use anyhow::{Error, Result};
use async_trait::async_trait;


// Should eventually be a trait that can be implemented for different file systems
// and backends
#[async_trait]
pub trait AsyncCrudFs {
    // C is for Create

    /// Create a new file in the CRUD filesystem's backend
    /// # Arguments
    /// * `file_object` - The file object to create
    /// # Returns
    /// * `Result<([u8; 32], u64), Error>` - The key and timestamp of the file in the filesystem
    async fn create_file(
        &mut self,
        file_object: FileObject,
    ) -> Result<([u8; 32], u64), Error>;

    // R is for Read

    /// Read a file from the CRUD filesystem's backend by key
    /// # Arguments
    /// * `key` - The key of the file to read
    /// # Returns
    /// * `Result<FileObject, Error>` - The file object
    async fn read_file(&mut self, key: [u8; 32]) -> Result<FileObject, Error>;

    // fn update_file(
    //     &mut self,
    //     file: File,
    //     metadata: HashMap<String, String>,
    // ) -> Result<FileObject, Error>;
    // fn delete_file(&mut self, file: File) -> Result<FileObject, Error>;
}