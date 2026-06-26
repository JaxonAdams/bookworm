use crate::{
    model::{Book, TBREntry},
    utils::print_tbr_table,
};
use rusqlite::{Connection, Result};

pub fn list_all_in_tbr(connection: &Connection) -> Result<()> {
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
                num_pages: row.get::<_, i32>(5)?,
            },
        })
    })?;

    let tbr: Vec<TBREntry> = tbr_iter.collect::<Result<Vec<_>, _>>()?;
    print_tbr_table(&tbr);

    Ok(())
}
