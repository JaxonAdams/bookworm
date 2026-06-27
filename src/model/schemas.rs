// TODO: reevaluate schema -- do we really need two tables?
// Keeping in mind TBR, progress tracking (currently reading, completed, not started, DNF,
// on-pause) and that users may want to re-read a book and add it to TBR when already
// completed.

// TODO: add 'created_at' field (default current timestamp)
// TODO: add 'status' field (default 'unread' or similar; include 'in progress', 'completed',
//      'DNF', 'on pause')
// TODO: add 'in_tbr' field (boolean; default false)

pub const BOOKS_TABLE_SCHEMA: &str = "
    id        INTEGER PRIMARY KEY,
    title     TEXT NOT NULL, 
    author    TEXT NOT NULL,
    num_pages INTEGER CHECK (num_pages IS NULL OR num_pages > 0),

    CONSTRAINT unique_book_entry UNIQUE (title, author)
";
