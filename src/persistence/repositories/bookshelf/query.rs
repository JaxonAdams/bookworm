use crate::model::Book;
use crate::utils::print_books_table;
use rusqlite::{Connection, Result};

pub fn list_all_books(connection: &Connection) -> Result<()> {
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
    print_books_table(&books);

    Ok(())
}
