#[derive(Debug)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub num_pages: Option<i32>,
    pub in_tbr: bool,
    pub added_to_tbr_at: Option<String>,
}
