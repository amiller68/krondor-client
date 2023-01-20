// main.rs: Our

mod args;
use clap::Parser;


fn main() {
    // Parse command line arguments
    let args = args::AppArgs::parse();
    // Determine which subcommand was called
    match args.subcmd {
        args::SubcommandType::Init(_) => {
            // Check if there is a manifest file

        }
        args::SubcommandType::Sync(_) => {
            println!("Sync subcommand called");
        }
    }
}
