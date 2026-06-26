use clap::{Parser, Subcommand};
use rusqlite::Connection;
use std::result::Result;

use crate::persistence::{create_book, delete_book, list_all_books};

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

    /// Manually ask Bookworm to track a book
    Add {
        /// The title of the book
        #[arg(short, long)]
        title: String,

        /// The author of the book
        #[arg(short, long)]
        author: String,

        /// The number of pages in the book
        #[arg(short, long)]
        num_pages: i32,
    },

    /// Remove a book
    Remove {
        /// The title of the book
        title: String,
    },
}

pub fn execute_cmd(
    cli: &Cli,
    db_connection: &Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    match &cli.command {
        TopLevelCommands::Book { action } => match action {
            BookCommands::List {} => list_all_books(db_connection)?,
            BookCommands::Add {
                title,
                author,
                num_pages,
            } => create_book(db_connection, title, author, num_pages)?,
            BookCommands::Remove { title } => delete_book(db_connection, title)?,
        },
    }

    Ok(())
}
