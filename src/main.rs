use rusqlite::Result;

mod config;
mod model;
mod persistence;

use persistence::{create_book, init_db, list_all_books};

fn main() -> Result<()> {
    println!("Hello, world!");

    let connection = init_db().unwrap();

    // FOR TESTING: insert test books
    create_book(
        &connection,
        &String::from("Les Miserables"),
        &String::from("Victor Hugo"),
        1218,
    )?;

    create_book(
        &connection,
        &String::from("The Hobbit"),
        &String::from("J.R.R. Tolkien"),
        300,
    )?;

    create_book(
        &connection,
        &String::from("Ender's Game"),
        &String::from("Orson Scott Card"),
        324,
    )?;

    // FOR TESTING: list all books in the table
    list_all_books(&connection)?;

    Ok(())
}
