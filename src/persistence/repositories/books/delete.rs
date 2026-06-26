use rusqlite::{Connection, params};
use std::result::Result;

use crate::{persistence::DatabaseError, utils::log_debug};

pub fn delete_book(connection: &Connection, title: &str) -> Result<(), DatabaseError> {
    log_debug(format!("Deleting book with title: '{}'", title).as_str());

    let rows_affected = connection.execute("DELETE FROM books WHERE title = ?1", params![title])?;

    if rows_affected == 0 {
        return Err(DatabaseError::BookNotFound(title.to_string()));
    }

    log_debug("Successfully completed DB operation.");
    Ok(())
}
