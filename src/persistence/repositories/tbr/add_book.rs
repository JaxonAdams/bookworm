use rusqlite::{Connection, Result, params};

use crate::persistence::bookshelf;
use crate::utils::log_debug;

pub fn add_book(
    connection: &Connection,
    title: &str,
    author: &str,
    num_pages: Option<i32>,
) -> Result<()> {
    // First make sure the book exists on the bookshelf
    bookshelf::add_book(connection, title, author, num_pages)?;

    // Add the book to the TBR list
    connection.execute(
        "INSERT INTO tbr (book_id)
         SELECT id FROM books WHERE title = ?1 AND author = ?2;
        ",
        params![title, author],
    )?;
    log_debug(&format!("Inserted new book: '{}, by {}'", title, author));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::setup_test_db;

    #[test]
    fn test_add_book_inserts_successfully() {
        let conn = setup_test_db();
        let result = add_book(&conn, "Dune", "Frank Herbert", Some(412));
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_book_inserts_successfully_no_num_pages() {
        let conn = setup_test_db();
        let result = add_book(&conn, "Dune", "Frank Herbert", None);
        assert!(result.is_ok());
    }
}
