use clap::Subcommand;
use rusqlite::Connection;
use std::result::Result;

use crate::{cli::Cli, persistence::bookshelf, persistence::tbr};

#[derive(Subcommand)]
pub enum TopLevelCommands {
    /// Manage books tracked by Bookworm
    Bookshelf {
        #[command(subcommand)]
        action: BookshelfCommands,
    },

    /// Manage books in your TBR (to-be-read) list
    TBR {
        #[command(subcommand)]
        action: TBRCommands,
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

#[derive(Subcommand)]
pub enum TBRCommands {
    /// List all books currently in your TBR (to-be-read) list
    List {},

    /// Add a book to your TBR (to-be-read) list
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
}

pub fn execute_cmd(
    cli: &Cli,
    db_connection: &Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    match &cli.command {
        TopLevelCommands::Bookshelf { action } => match action {
            BookshelfCommands::List {} => bookshelf::list_all_books(db_connection)?,
            BookshelfCommands::Add {
                title,
                author,
                num_pages,
            } => bookshelf::add_book(db_connection, title, author, num_pages)?,
            BookshelfCommands::Remove { title } => bookshelf::delete_book(db_connection, title)?,
        },
        TopLevelCommands::TBR { action } => match action {
            TBRCommands::List {} => tbr::list_all_in_tbr(db_connection)?,
            TBRCommands::Add {
                title,
                author,
                num_pages,
            } => tbr::add_book(db_connection, title, author, num_pages)?,
        },
    }

    Ok(())
}
