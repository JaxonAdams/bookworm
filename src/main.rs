use rusqlite::{Connection, Result};
use std::{env, fs, path::PathBuf};

mod model;

use model::Book;

const APP_NAME: &str = "bookworm";

fn main() {
    println!("Hello, world!");

    let test_book = Book {
        id: 1,
        title: String::from("Les Miserables"),
        author: String::from("Victor Hugo"),
        num_pages: 1218,
    };

    println!("Test Book: {:?}", test_book);

    let _connection = init_db();
}

/// Initializes the connection and ensures the tables exist
fn init_db() -> Result<Connection> {
    let db_path = get_db_path();
    println!("Database path: {:?}", db_path);

    let connection = Connection::open(db_path)?;

    // TODO: create the table / ensure it exists

    println!("Database initialized successfully!");
    Ok(connection)
}

/// Determines where the database file should live
fn get_db_path() -> PathBuf {
    let storage_file_name = "storage.db";

    if env::var("DEV_MODE").is_ok() {
        println!("Running in Development Mode...");

        let dev_dir = PathBuf::from("data");
        fs::create_dir_all(&dev_dir).unwrap();

        return dev_dir.join(storage_file_name);
    }

    if let Some(project_dirs) = directories::ProjectDirs::from("com", "jaxonadams", APP_NAME) {
        let prod_dir = project_dirs.config_dir();
        fs::create_dir_all(prod_dir).unwrap();

        return prod_dir.join(storage_file_name);
    }

    // FALLBACK
    PathBuf::from(storage_file_name)
}
