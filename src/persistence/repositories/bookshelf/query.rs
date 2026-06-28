use crate::model::Book;
use rusqlite::{Connection, Result};

pub fn list_all_books(connection: &Connection) -> Result<Vec<Book>> {
    let mut stmt = connection
        .prepare("SELECT id, title, author, num_pages, in_tbr, added_to_tbr_at FROM books")?;

    let book_iter = stmt.query_map([], |row| {
        Ok(Book {
            id: row.get(0)?,
            title: row.get(1)?,
            author: row.get(2)?,
            num_pages: row.get::<_, Option<i32>>(3)?,
            in_tbr: row.get(4)?,
            added_to_tbr_at: row.get::<_, Option<String>>(5)?,
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
        conn.execute(
            "INSERT INTO books (title, author, num_pages) VALUES (?1, ?2, ?3)",
            params!["Les Miserables", "Victor Hugo", Option::<i32>::None],
        )
        .unwrap();

        conn
    }

    #[test]
    fn test_list_all_in_tbr_queries_successfully() {
        let conn = seed_tbr();
        let results = list_all_books(&conn).expect("Failed to fetch bookshelf");

        // Assert the total count
        assert_eq!(results.len(), 3);

        // Assert specific details of the first entry
        assert_eq!(results[0].title, "Dune");
        assert_eq!(results[0].author, "Frank Herbert");
        assert_eq!(results[0].num_pages, Some(412));
        assert_eq!(results[0].in_tbr, false);
        assert_eq!(results[0].added_to_tbr_at, None);

        // Assert specific details of the second entry
        assert_eq!(results[1].title, "The Lord of the Rings");
        assert_eq!(results[1].author, "J.R.R. Tolkien");
        assert_eq!(results[1].num_pages, Some(800));
        assert_eq!(results[1].in_tbr, false);
        assert_eq!(results[1].added_to_tbr_at, None);

        // Assert specific details of the third entry
        assert_eq!(results[2].title, "Les Miserables");
        assert_eq!(results[2].author, "Victor Hugo");
        assert_eq!(results[2].num_pages, None);
        assert_eq!(results[2].in_tbr, false);
        assert_eq!(results[2].added_to_tbr_at, None);
    }
}
