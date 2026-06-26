#[derive(Debug)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub num_pages: i32,
}

#[derive(Debug)]
pub struct TBREntry {
    pub id: i32,
    pub created_at: String,
    pub book: Book,
}
