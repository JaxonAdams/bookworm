use crate::{config::APP_NAME, utils::log_debug};
use rusqlite::{Connection, Result};
use std::{env, fs, path::PathBuf};

/// Initializes the connection and ensures the tables exist
pub fn init_db() -> Result<Connection> {
    let db_path = get_db_path();
    log_debug(format!("Database path: {:?}", db_path).as_str());

    let connection = Connection::open(db_path)?;

    set_up_tables(&connection)?;

    log_debug("Database initialized successfully!");
    Ok(connection)
}

/// Determines where the database file should live
fn get_db_path() -> PathBuf {
    let storage_file_name = "storage.db";

    if env::var("DEV_MODE").is_ok() {
        log_debug("Running in Development Mode...");

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

/// Creates the needed tables if they don't exist
fn set_up_tables(connection: &Connection) -> Result<()> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS bookshelf (
            id        INTEGER PRIMARY KEY,
            title     TEXT NOT NULL,
            author    TEXT NOT NULL,
            num_pages INTEGER NOT NULL CHECK (num_pages > 0),

            CONSTRAINT unique_book_entry UNIQUE (title, author)
        );",
        [],
    )?;
    Ok(())
}
