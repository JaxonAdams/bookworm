use rusqlite::{Connection, Result, params};

use crate::utils::log_debug;

pub fn create_book(
    connection: &Connection,
    title: &str,
    author: &str,
    num_pages: &i32,
) -> Result<()> {
    connection.execute(
        "INSERT INTO books (title, author, num_pages)
            VALUES (?1, ?2, ?3)
            ON CONFLICT(title, author)
            DO UPDATE SET num_pages = excluded.num_pages;
        ",
        params![title, author, num_pages],
    )?;
    log_debug(format!("Inserted new book: '{}, by {}'", title, author).as_str());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::setup_test_db;

    #[test]
    fn test_create_book_inserts_successfully() {
        let conn = setup_test_db();
        let result = create_book(&conn, "Dune", "Frank Herbert", &412);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_book_upserts_successfully() {
        let conn = setup_test_db();
        let result2 = create_book(&conn, "Dune", "Frank Herbert", &42);
        let result1 = create_book(&conn, "Dune", "Frank Herbert", &412);

        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }

    #[test]
    fn test_create_book_enforces_num_pages_check() {
        let conn = setup_test_db();
        let result = create_book(&conn, "Dune", "Frank Herbert", &-412);
        assert!(!result.is_ok());
    }
}
