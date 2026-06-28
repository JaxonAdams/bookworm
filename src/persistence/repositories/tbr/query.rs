use crate::model::Book;
use rusqlite::{Connection, Result};

pub fn list_all_in_tbr(connection: &Connection) -> Result<Vec<Book>> {
    let mut stmt = connection.prepare(
        "SELECT id, title, author, num_pages, in_tbr, added_to_tbr_at
         FROM books
         WHERE in_tbr = 1;",
    )?;

    let tbr_iter = stmt.query_map([], |row| {
        Ok(Book {
            id: row.get(0)?,
            title: row.get(1)?,
            author: row.get(2)?,
            num_pages: row.get::<_, Option<i32>>(3)?,
            in_tbr: row.get(4)?,
            added_to_tbr_at: row.get::<_, Option<String>>(5)?,
        })
    })?;

    let tbr: Vec<Book> = tbr_iter.collect::<Result<Vec<_>, _>>()?;
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
            "INSERT INTO books (title, author, num_pages, in_tbr, added_to_tbr_at) 
             VALUES (?1, ?2, ?3, 1, CURRENT_TIMESTAMP)",
            params!["Dune", "Frank Herbert", 412],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO books (title, author, num_pages, in_tbr, added_to_tbr_at) 
             VALUES (?1, ?2, ?3, 1, CURRENT_TIMESTAMP)",
            params!["Les Miserables", "Victor Hugo", Option::<i32>::None],
        )
        .unwrap();

        // NOTE: NOT IN TBR! Just a book.
        conn.execute(
            "INSERT INTO books (title, author, num_pages) 
             VALUES (?1, ?2, ?3)",
            params!["Moby Dick", "Herman Melville", Option::<i32>::None],
        )
        .unwrap();

        conn
    }

    #[test]
    fn test_list_all_in_tbr_queries_successfully() {
        let conn = seed_tbr();
        let results = list_all_in_tbr(&conn).expect("Failed to fetch TBR list");

        // Assert the total count
        assert_eq!(results.len(), 2);

        // Assert specific details of the first entry
        assert_eq!(results[0].title, "Dune");
        assert_eq!(results[0].author, "Frank Herbert");
        assert_eq!(results[0].num_pages, Some(412));

        // Assert specific details of the second entry
        assert_eq!(results[1].title, "Les Miserables");
        assert_eq!(results[1].author, "Victor Hugo");
        assert_eq!(results[1].num_pages, None);
    }
}
