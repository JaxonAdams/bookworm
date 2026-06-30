use core::result::Result::Err;
use rusqlite::{Connection, params};
use std::result::Result;

use crate::{persistence::DatabaseError, utils::log_debug};

pub fn delete_book(connection: &Connection, title: &str) -> Result<(), DatabaseError> {
    log_debug(&format!("Deleting book with title: '{}'", title));

    let rows_affected = connection.execute(
        "UPDATE books
         SET in_tbr = 0,
             added_to_tbr_at = NULL
         WHERE title = ?1;",
        params![title],
    )?;

    if rows_affected == 0 {
        return Err(DatabaseError::BookNotFound(title.to_string()));
    }

    log_debug("Successfully completed DB operation.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::setup_test_db;

    #[test]
    fn test_delete_existing_book_succeeds() {
        let conn = setup_test_db();
        conn.execute(
            "INSERT INTO books (title, author, in_tbr, added_to_tbr_at) VALUES (?1, ?2, 1, CURRENT_TIMESTAMP);",
            params!["Dune", "Frank Herbert"],
        ).unwrap();

        let result = delete_book(&conn, "Dune");
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_nonexistent_book_returns_book_not_found() {
        let conn = setup_test_db();

        let result = delete_book(&conn, "Nonexistent");
        assert!(matches!(
            result,
            Err(DatabaseError::BookNotFound(ref t)) if t == "Nonexistent"
        ));
    }
}
