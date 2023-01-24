// main.rs: Our

use std::io::stdin;
use std::path::PathBuf;

// Module imports
mod manifest;
mod manifest;
mod sync;
// mod lib;

use manifest::Manifest;
use sync::Sync;

const DEFAULT_MANIFEST_PATH: &str = "../content/manifest.json";

/// Init the manifest file and sync the filesystem
fn main() {
    // Try to load the manifest and create a new one if it doesn't exist
    let manifest_path = PathBuf::from(DEFAULT_MANIFEST_PATH);
    let manifest = match Manifest::load(&manifest_path) {
        Ok(manifest) => manifest,
        Err(_) => {
            // Prompt the user for the backend address
            println!("No manifest found. Would you like to create a new one? [y/n]");
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            if input.trim() == "y" {
                println!("Please enter the address of the backend contract:");
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();
                let backend_address = input.trim().to_string();
                let manifest = Manifest::new(backend_address);
                manifest.save(&manifest_path).unwrap();
                manifest
            } else {
                println!("Exiting...");
                std::process::exit(0);
            }
        }
    };
    // Create a new Sync instance
    let sync = Sync::new(manifest, PathBuf::from("../content"));
    // Sync the filesystem
    sync.sync().unwrap_or_else(|e| {
        println!("Error: {}", e);
        std::process::exit(1);
    });
}
