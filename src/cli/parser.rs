use clap::Parser;

use crate::cli::commands::TopLevelCommands;

/// Bookworm! A tool by readers, for readers!
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Enable verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: TopLevelCommands,
}
