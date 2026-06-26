use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};

use crate::persistence::list_all_books;

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

#[derive(Subcommand)]
pub enum TopLevelCommands {
    /// Manage books tracked by Bookworm
    Book {
        #[command(subcommand)]
        action: BookCommands,
    },
}

#[derive(Subcommand)]
pub enum BookCommands {
    /// List all books currently tracked by Bookworm
    List {},
}

pub fn execute_cmd(cli: &Cli, db_connection: &Connection) -> Result<()> {
    match &cli.command {
        TopLevelCommands::Book { action } => match action {
            BookCommands::List {} => list_all_books(db_connection)?,
        },
    }

    Ok(())
}
