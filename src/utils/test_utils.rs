use rusqlite::Connection;

pub fn setup_test_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute(
        "CREATE TABLE books (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            author TEXT NOT NULL,
            num_pages INTEGER NOT NULL CHECK (num_pages > 0),
            CONSTRAINT unique_book_entry UNIQUE (title, author)
        )",
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
