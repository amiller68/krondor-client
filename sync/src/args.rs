// // args.rs: command line argument parsing
// use clap::{Args, Parser, Subcommand};
//
// /// Sync Arguments
// /// Subcommands:
// ///     init: initialize the manifest
// ///     sync: sync the local filesystem with the manifest, backend, and IPFS
// ///     TODO daemon: repeatedly sync the local filesystem with the manifest, backend, and IPFS
// #[derive(Debug, Parser)]
// #[clap(author, version, about, long_about = None)]
// pub struct AppArgs {
//     #[clap(subcommand)]
//     pub subcmd: SubcommandType,
// }
//
// /* Subcommands */
//
// #[derive(Debug, Subcommand)]
// pub enum SubcommandType {
//     /// Initialize the manifest
//     #[clap(about = "Initialize the manifest")]
//     Init(InitArgs),
//     /// Sync the local filesystem with the manifest, backend, and IPFS
//     #[clap(about = "Sync the local filesystem with the manifest, backend, and IPFS")]
//     Sync(SyncArgs),
// }
//
//
// /* Init Arguments */
// #[derive(Debug, Args)]
// pub struct InitArgs {}
//
// /* Sync Arguments */
// #[derive(Debug, Args)]
// pub struct SyncArgs {}
//
