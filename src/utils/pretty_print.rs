use crate::model::Book;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, Color, Row, Table};

pub fn print_books_table(books: &[Book]) {
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

    for book in books {
        let id_cell = Cell::new(&book.id);
        let title_cell = Cell::new(&book.title);
        let author_cell = Cell::new(&book.author);
        let num_pages_cell = Cell::new(&book.num_pages);

        table.add_row(vec![id_cell, title_cell, author_cell, num_pages_cell]);
    }

    println!("\n{}", table);
}
