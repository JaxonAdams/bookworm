use rusqlite::Connection;

use crate::model::schemas;

pub fn setup_test_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute(
        &format!(
            "CREATE TABLE IF NOT EXISTS books ({});",
            schemas::BOOKS_TABLE_SCHEMA,
        ),
        [],
    )
    .unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tbr (
            id INTEGER PRIMARY KEY,
            book_id INTEGER NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,

            FOREIGN KEY (book_id) REFERENCES books (id)
                ON DELETE CASCADE
        );",
        [],
    )
    .unwrap();

    conn
}
