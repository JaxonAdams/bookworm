use crate::model::{Book, TBREntry};
use rusqlite::{Connection, Result};

pub fn list_all_in_tbr(connection: &Connection) -> Result<Vec<TBREntry>> {
    let mut stmt = connection.prepare(
        "SELECT tbr.id, tbr.created_at, b.id, b.title, b.author, b.num_pages 
         FROM tbr
         LEFT JOIN books b ON b.id = tbr.book_id",
    )?;

    let tbr_iter = stmt.query_map([], |row| {
        Ok(TBREntry {
            id: row.get(0)?,
            created_at: row.get(1)?,
            book: Book {
                id: row.get(2)?,
                title: row.get(3)?,
                author: row.get(4)?,
                num_pages: row.get::<_, Option<i32>>(5)?,
            },
        })
    })?;

    let tbr: Vec<TBREntry> = tbr_iter.collect::<Result<Vec<_>, _>>()?;
    Ok(tbr)
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
        conn.execute(
            "INSERT INTO tbr (book_id)
         SELECT id FROM books WHERE title = ?1 AND author = ?2;
        ",
            params!["Dune", "Frank Herbert"],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO tbr (book_id)
         SELECT id FROM books WHERE title = ?1 AND author = ?2;
        ",
            params!["The Lord of the Rings", "J.R.R. Tolkien"],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO tbr (book_id)
         SELECT id FROM books WHERE title = ?1 AND author = ?2;
        ",
            params!["Les Miserables", "Victor Hugo"],
        )
        .unwrap();

        conn
    }

    #[test]
    fn test_list_all_in_tbr_queries_successfully() {
        let conn = seed_tbr();
        let results = list_all_in_tbr(&conn).expect("Failed to fetch TBR list");

        // Assert the total count
        assert_eq!(results.len(), 3);

        // Assert specific details of the first entry
        assert_eq!(results[0].book.title, "Dune");
        assert_eq!(results[0].book.author, "Frank Herbert");
        assert_eq!(results[0].book.num_pages, Some(412));

        // Assert specific details of the second entry
        assert_eq!(results[1].book.title, "The Lord of the Rings");
        assert_eq!(results[1].book.author, "J.R.R. Tolkien");
        assert_eq!(results[1].book.num_pages, Some(800));

        // Assert specific details of the second entry
        assert_eq!(results[2].book.title, "Les Miserables");
        assert_eq!(results[2].book.author, "Victor Hugo");
        assert_eq!(results[2].book.num_pages, None);
    }
}
