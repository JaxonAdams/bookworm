use clap::Parser;
use rusqlite::Result;

mod cli;
mod config;
mod model;
mod persistence;
mod utils;

use cli::{Cli, execute_cmd};
use persistence::init_db;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let connection = init_db().unwrap();

    execute_cmd(&cli, &connection)?;
    Ok(())
}
