// #[allow(unused_imports)]
//
// use anyhow::{Error, Result, anyhow};
// use serde::{Deserialize, Serialize};
// use std::{collections::HashMap, fs::File, io::{Write, Read}, path::PathBuf};
// use utils::hash::hash_path;
//
// use crate::types::{
//     cid::Cid,
//     crud_file::CrudFile
// };
// use crate::utils::hash::hash_path;
//
//
// // TODO (amiller68): Obake this
// /// Our manifest data structure
// /// Version 1.0.0 Specification
// /// # Fields
// /// * `version_number` - The major version number of the manifest
// /// * `files` - The list of files in the manifest
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Manifest {
//     /// The version of the manifest
//     pub version: String,
//     /// The address of the IPFS node
//     pub files: HashMap<[u8; 32], CrudFile>,
// }
//
// /// Manifest - Represents the manifest file
// impl Manifest {
//     /// Create a new Manifest instance
//     /// # Arguments
//     pub fn new() -> Self {
//         Self {
//             version: String::from("1.0.0"),
//             files: HashMap::new(),
//         }
//     }
//
//     // C - Create
//     /// Create a new file in the manifest
//     /// # Arguments
//     /// * `crud_file` - The file to add to the manifest
//     /// # Returns
//     /// * `Result<(), Error>` - The result
//     pub fn create_file(&mut self, crud_file: CrudFile) -> Result<(), anyhow::Error> {
//         let key = hash_path(&crud_file.path)?;
//         if self.files.contains_key(&key) {
//             anyhow!("File already exists in manifest")
//         }
//         self.files.insert(key, crud_file);
//         Ok(())
//     }
//
//     // R - Read
//     /// Read a file from the manifest
//     /// # Arguments
//     /// * `path` - The path to the file
//     /// # Returns
//     /// * `Result<CrudFile, Error>` - The result
//     pub fn read_file(&self, path: &PathBuf) -> Result<CrudFile, anyhow::Error> {
//         let key = hash_path(path)?;
//         match self.files.get(&key) {
//             Some(crud_file) => Ok(crud_file.clone()),
//             None =>             anyhow!("File already exists in manifest")
//         }
//     }
//
//     // U - Update
//     /// Update a file in the manifest
//     /// # Arguments
//     /// * `crud_file` - The file to update in the manifest
//     /// # Returns
//     /// * `Result<(), Error>` - The result
//     pub fn update_file(&mut self, crud_file: CrudFile) -> Result<(), anyhow::Error> {
//         let key = hash_path(&crud_file.path)?;
//         if !self.files.contains_key(&key) {
//
//         }
//         self.files.insert(key, crud_file);
//         Ok(())
//     }
//
//     // D - Delete
//     /// Delete a file from the manifest
//     /// # Arguments
//     /// * `path` - The path to the file
//     /// # Returns
//     /// * `Result<(), Error>` - The result
//     pub fn delete_file(&mut self, path: &PathBuf) -> Result<(), anyhow::Error> {
//         let key = hash_path(path)?;
//         if !self.files.contains_key(&key) {
//             Err(anyhow!("File does not exist in manifest")).unwrap();
//         }
//         self.files.remove(&key);
//         Ok(())
//     }
// }
