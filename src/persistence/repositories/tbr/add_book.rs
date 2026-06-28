use rusqlite::{Connection, Result, params};

use crate::utils::log_debug;

pub fn add_book(
    connection: &Connection,
    title: &str,
    author: &str,
    num_pages: Option<i32>,
) -> Result<()> {
    // Add the book to the TBR list
    connection.execute(
        "INSERT INTO books (title, author, num_pages, in_tbr, added_to_tbr_at)
         VALUES (?1, ?2, ?3, 1, CURRENT_TIMESTAMP)
         ON CONFLICT(title, author)
         DO UPDATE SET 
            num_pages = excluded.num_pages,
            added_to_tbr_at = CASE 
                WHEN books.in_tbr = 0 THEN excluded.added_to_tbr_at 
                ELSE books.added_to_tbr_at 
            END,
            in_tbr = excluded.in_tbr;",
        params![title, author, num_pages],
    )?;
    log_debug(&format!("Book now in user TBR: '{}, by {}'", title, author));

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
