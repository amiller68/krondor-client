use clap::Parser;
use std::io::stdin;
use std::path::PathBuf;
use tokio;
use lazy_static::lazy_static;

mod manifest;
mod args;
use sync::{
    crud_fs::crud_fs::CrudFs,
    types::{
        cid::Cid,
        metadata::Metadata,
        crud_file::CrudFile
    }
};

use crate::{
    manifest::Manifest,
    args::{
        SyncArgs,
        SubcommandType
    }
};

lazy_static! {
    static ref DEFAULT_MANIFEST_PATH: PathBuf = PathBuf::from("manifest.json");
}

/// Init the manifest file and sync the filesystem
#[tokio::main]
async fn main() {
    // Parse the command line arguments
    let args = SyncArgs::parse();
    // Execute the subcommand
    match args.subcommand {
        SubcommandType::Create(args) => {
            println!("Creating file: {}", args.path.display());
            // Get the path to the file to create
            let path = args.path;
            // Get the Metadata from the args
            let metadata: Metadata = match args.metadata {
                Some(metadata) => serde_json::from_str(&metadata).unwrap(),
                None => serde_json::from_str("{}").unwrap(),
            };
            // Get the manifest
            let mut manifest: Manifest = match args.manifest {
                Some(manifest_path) => Manifest::read(&manifest_path).unwrap(),
                None => Manifest::read(&DEFAULT_MANIFEST_PATH).unwrap_or_else(|_| {
                    Manifest::new("".to_string()).write(&DEFAULT_MANIFEST_PATH).unwrap();
                    println!("Manifest Uninitialized");
                    println!("I went and made a template ror you, go fill it out!");
                    std::process::exit(0);
                }),
            };
            // Check if the file already exists
            if manifest.contains(&path).unwrap() {
                println!("File already exists in the manifest");
                println!("Eventually you will be able to update the file (this is a todo)");
                std::process::exit(0);
            }
            // Initialize the CrudFs
            let mut crud_fs = CrudFs::new(manifest.contract_address.clone());
            // Get the CID from the path
            let cid = Cid::try_from(&path).unwrap();
            println!("-> Creating with CID: {}", cid.to_string());
            println!("-> Creating with Metadata: {}", serde_json::to_string(&metadata).unwrap());
            // Create a new CrudFile with CrudFs
            let crud_file= crud_fs.create(
                path, cid, metadata
            ).await.unwrap_or_else(|e| {
                println!("Could not push to CrudFs: {}", e);
                std::process::exit(0);
            });
            // Add the CrudFile to the manifest
            let _ = manifest.add(crud_file.clone()).unwrap();
            // Write the manifest to the manifest file
            let _ = manifest.write(&DEFAULT_MANIFEST_PATH).unwrap();
        }
    }
}
