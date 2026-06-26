use crate::model::Book;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, Color, Row, Table};
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

    let mut table = Table::new();

    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);

    table.set_header(Row::from(vec![
        Cell::new("ID")
            .add_attribute(Attribute::Bold)
            .fg(Color::Cyan),
        Cell::new("Title")
            .add_attribute(Attribute::Bold)
            .fg(Color::Cyan),
        Cell::new("Author")
            .add_attribute(Attribute::Bold)
            .fg(Color::Cyan),
        Cell::new("Number of Pages")
            .add_attribute(Attribute::Bold)
            .fg(Color::Cyan),
    ]));

    let mut has_books = false;
    for book_result in book_iter {
        let book = book_result?;
        has_books = true;

        let id_cell = Cell::new(&book.id);
        let title_cell = Cell::new(&book.title);
        let author_cell = Cell::new(&book.author);
        let num_pages_cell = Cell::new(&book.num_pages);

        table.add_row(vec![id_cell, title_cell, author_cell, num_pages_cell]);
    }

    if !has_books {
        println!("\nNo books added yet! Get reading!");
    } else {
        println!("\n{}", table);
    }

    Ok(())
}
