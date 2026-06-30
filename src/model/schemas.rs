// TODO: add 'status' field (default 'unread' or similar; include 'in progress', 'completed',
//      'DNF', 'on pause')
//
// TODO: timestamps in local time
pub const BOOKS_TABLE_SCHEMA: &str = "
    id              INTEGER PRIMARY KEY,
    title           TEXT NOT NULL, 
    author          TEXT NOT NULL,
    num_pages       INTEGER CHECK (num_pages IS NULL OR num_pages > 0),
    in_tbr          BOOLEAN DEFAULT FALSE,
    added_to_tbr_at TEXT CHECK (
                        (in_tbr = 0 AND added_to_tbr_at IS NULL)
                        OR (in_tbr = 1 AND DATETIME(added_to_tbr_at) IS NOT NULL)
                    ),
    created_at      TEXT DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT unique_book_entry UNIQUE (title, author)
";
