mod config;
mod model;
mod persistence;

use model::Book;
use persistence::init_db;

fn main() {
    println!("Hello, world!");

    let test_book = Book {
        id: 1,
        title: String::from("Les Miserables"),
        author: String::from("Victor Hugo"),
        num_pages: 1218,
    };

    println!("Test Book: {:?}", test_book);

    let _connection = init_db();
}
