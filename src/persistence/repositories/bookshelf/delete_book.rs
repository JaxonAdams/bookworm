use rusqlite::{Connection, params};
use std::result::Result;

use crate::{persistence::DatabaseError, utils::log_debug};

pub fn delete_book(connection: &Connection, title: &str) -> Result<(), DatabaseError> {
    log_debug(&format!("Deleting book with title: '{}'", title));

    let rows_affected = connection.execute("DELETE FROM books WHERE title = ?1", params![title])?;

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
            "INSERT INTO books (title, author, num_pages) VALUES (?1, ?2, ?3)",
            params!["Dune", "Frank Herbert", 412],
        )
        .unwrap();

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
