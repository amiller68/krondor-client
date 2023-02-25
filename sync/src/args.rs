use std::path::PathBuf;
// args.rs: command line argument parsing
use clap::{Args, Parser, Subcommand};

/// Sync Arguments
/// Subcommands:
///     init: initialize the manifest
///     sync: sync the local filesystem with the manifest, backend, and IPFS
///     TODO daemon: repeatedly sync the local filesystem with the manifest, backend, and IPFS
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct AppArgs {
    #[clap(subcommand)]
    pub subcommand: SubcommandType,
}

/* Subcommands */

#[derive(Debug, Subcommand)]
pub enum SubcommandType {
    /// Create a file in the configured backend
    Create(CreateArgs),
}


/* Init Arguments */
#[derive(Debug, Args)]
pub struct InitArgs {}

/* Sync Arguments */
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// The path to the file to create
    #[clap(short, long)]
    pub path: PathBuf,
    /// Metadata to store with the file
    #[clap(short, long)]
    pub metadata: Option<String>,
}

