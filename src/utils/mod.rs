mod log;
mod pretty_print;

pub use log::{log_debug, set_verbose};
pub use pretty_print::print_books_table;

#[cfg(test)]
pub mod test_utils;
