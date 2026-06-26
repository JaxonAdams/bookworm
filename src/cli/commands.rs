use clap::Subcommand;
use rusqlite::Connection;
use std::result::Result;

use crate::{
    cli::Cli,
    persistence::bookshelf::{add_book, delete_book, list_all_books},
};

#[derive(Subcommand)]
pub enum TopLevelCommands {
    /// Manage books tracked by Bookworm
    Bookshelf {
        #[command(subcommand)]
        action: BookshelfCommands,
    },
}

#[derive(Subcommand)]
pub enum BookshelfCommands {
    /// List all books currently tracked by Bookworm
    List {},

    /// Add a book to your bookshelf
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

    /// Remove a book from your bookshelf
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
        TopLevelCommands::Bookshelf { action } => match action {
            BookshelfCommands::List {} => list_all_books(db_connection)?,
            BookshelfCommands::Add {
                title,
                author,
                num_pages,
            } => add_book(db_connection, title, author, num_pages)?,
            BookshelfCommands::Remove { title } => delete_book(db_connection, title)?,
        },
    }

    Ok(())
}
