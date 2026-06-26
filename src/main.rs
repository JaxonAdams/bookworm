use clap::Parser;
use rusqlite::Result;

mod cli;
mod config;
mod model;
mod persistence;
mod utils;

use cli::{Cli, execute_cmd};
use persistence::init_db;
use utils::set_verbose;

fn main() -> Result<()> {
    let cli = Cli::parse();
    set_verbose(cli.verbose);

    let connection = init_db().unwrap();
    execute_cmd(&cli, &connection)?;
    Ok(())
}
