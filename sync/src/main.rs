
use std::io::stdin;
use std::path::PathBuf;
use tokio;
use crate::args::AppArgs;
use sync::types::crud_file::CrudFile;
use sync::crud_fs::crud_fs::CrudFs;
use crate::args::SubcommandType;
use clap::Parser;

mod args;

const DEFAULT_MANIFEST_PATH: &str = "../content/manifest.json";

/// Init the manifest file and sync the filesystem
#[tokio::main]
async fn main() {
    // Parse the command line arguments
    let args = AppArgs::parse();
    // Init CrudFs
    let crud_fs = CrudFs::default();
    // Execute the subcommand
    match args.subcommand {
        SubcommandType::Create(args) => {
            // Get the path to the file to create
            let path = args.path;
            // Get the metadata to store with the file
            let metadata = args.metadata;
            // Create a new CrudFile
            let mut crud_file = CrudFile::new(path).unwrap();
            // Add the metadata to the CrudFile
            if let Some(metadata) = metadata {
                // Create a hashmap from the metadata string
                let metadata = serde_json::from_str(&metadata).unwrap();
                crud_file.set_metadata(metadata);
            }
            // Create the file in the backend, store, and local
            crud_fs.create(crud_file).await.unwrap_or_else(|e| {
                println!("Error: {}", e);
                std::process::exit(1);
            });
        }
    }


    // Try to load the manifest and create a new one if it doesn't exist
    // let manifest_path = PathBuf::from(DEFAULT_MANIFEST_PATH);
    // let manifest = match Manifest::load(&manifest_path) {
    //     Ok(manifest) => manifest,
    //     Err(_) => {
    //         // Prompt the user for the backend address
    //         println!("No manifest found. Would you like to create a new one? [y/n]");
    //         let mut input = String::new();
    //         stdin().read_line(&mut input).unwrap();
    //         if input.trim() == "y" {
    //             println!("Please enter the address of the backend contract:");
    //             let mut input = String::new();
    //             stdin().read_line(&mut input).unwrap();
    //             let backend_address = input.trim().to_string();
    //             let manifest = Manifest::new(backend_address);
    //             manifest.save(&manifest_path).unwrap();
    //             manifest
    //         } else {
    //             println!("Exiting...");
    //             std::process::exit(0);
    //         }
    //     }
    // };
    // // Create a new Sync instance
    // let sync = Sync::new(manifest, PathBuf::from("../content"));
    // // Sync the filesystem
    // sync.sync().unwrap_or_else(|e| {
    //     println!("Error: {}", e);
    //     std::process::exit(1);
    // });
}
