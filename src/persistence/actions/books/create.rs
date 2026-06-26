use rusqlite::{Connection, Result, params};

pub fn create_book(
    connection: &Connection,
    title: &String,
    author: &String,
    num_pages: i32,
) -> Result<()> {
    connection.execute(
        "INSERT INTO books (title, author, num_pages)
            VALUES (?1, ?2, ?3)
            ON CONFLICT(title, author)
            DO UPDATE SET num_pages = excluded.num_pages;
        ",
        params![title, author, num_pages],
    )?;
    println!("Inserted new book: '{}, by {}'", title, author);

    Ok(())
}
