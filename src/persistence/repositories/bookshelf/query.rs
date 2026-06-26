use crate::model::Book;
use rusqlite::{Connection, Result};

pub fn list_all_books(connection: &Connection) -> Result<Vec<Book>> {
    let mut stmt = connection.prepare("SELECT id, title, author, num_pages FROM books")?;

    let book_iter = stmt.query_map([], |row| {
        Ok(Book {
            id: row.get(0)?,
            title: row.get(1)?,
            author: row.get(2)?,
            num_pages: row.get::<_, i32>(3)?,
        })
    })?;

    let books: Vec<Book> = book_iter.collect::<Result<Vec<_>, _>>()?;
    Ok(books)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::setup_test_db;
    use rusqlite::params;

    fn seed_tbr() -> Connection {
        let conn = setup_test_db();
        conn.execute(
            "INSERT INTO books (title, author, num_pages) VALUES (?1, ?2, ?3)",
            params!["Dune", "Frank Herbert", 412],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO books (title, author, num_pages) VALUES (?1, ?2, ?3)",
            params!["The Lord of the Rings", "J.R.R. Tolkien", 800],
        )
        .unwrap();

        conn
    }

    #[test]
    fn test_list_all_in_tbr_queries_successfully() {
        let conn = seed_tbr();
        let results = list_all_books(&conn).expect("Failed to fetch bookshelf");

        // Assert the total count
        assert_eq!(results.len(), 2);

        // Assert specific details of the first entry
        assert_eq!(results[0].title, "Dune");
        assert_eq!(results[0].author, "Frank Herbert");
        assert_eq!(results[0].num_pages, 412);

        // Assert specific details of the second entry
        assert_eq!(results[1].title, "The Lord of the Rings");
        assert_eq!(results[1].author, "J.R.R. Tolkien");
        assert_eq!(results[1].num_pages, 800);
    }
}
