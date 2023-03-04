use std::path::PathBuf;
use clap::{Args, Parser, Subcommand};

/// Sync Arguments
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct SyncArgs {
    #[clap(subcommand)]
    pub subcommand: SubcommandType,
}

#[derive(Debug, Subcommand)]
pub enum SubcommandType {
    /// Create a file in the configured backend
    Create(CreateArgs),
}

/* Subcommands */

/* Sync Arguments */
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// The path to the file to create
    #[clap(short, long)]
    pub path: PathBuf,
    /// Metadata to store with the file
    #[clap(short, long)]
    pub metadata: Option<String>,
    /// Path to the manifest file
    #[clap(long)]
    pub manifest: Option<PathBuf>,
}
