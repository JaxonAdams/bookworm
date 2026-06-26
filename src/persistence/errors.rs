#[derive(Debug)]
pub enum DatabaseError {
    Rusqlite(rusqlite::Error),
    BookNotFound(String),
}

impl From<rusqlite::Error> for DatabaseError {
    fn from(error: rusqlite::Error) -> Self {
        DatabaseError::Rusqlite(error)
    }
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::Rusqlite(e) => write!(f, "Database error: {}", e),
            DatabaseError::BookNotFound(title) => write!(f, "Book not found: '{}'", title),
        }
    }
}

impl std::error::Error for DatabaseError {}
