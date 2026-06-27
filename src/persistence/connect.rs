use crate::{config::APP_NAME, model::schemas, utils::log_debug};
use rusqlite::{Connection, Result};
use std::{env, fs, path::PathBuf};

/// Initializes the connection and ensures the tables exist
pub fn init_db() -> Result<Connection> {
    let db_path = get_db_path();
    log_debug(format!("Database path: {:?}", db_path).as_str());

    let connection = Connection::open(db_path)?;

    connection.execute("PRAGMA foreign_keys = ON;", [])?;
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
        &format!(
            "CREATE TABLE IF NOT EXISTS books ({});",
            schemas::BOOKS_TABLE_SCHEMA,
        ),
        [],
    )?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS tbr (
            id INTEGER PRIMARY KEY,
            book_id INTEGER NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,

            FOREIGN KEY (book_id) REFERENCES books (id)
                ON DELETE CASCADE
        );",
        [],
    )?;

    Ok(())
}
