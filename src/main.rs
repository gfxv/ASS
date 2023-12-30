#![allow(unused)]
mod cli;
mod storage;
mod core;

use crate::cli::cli::Cli;
use crate::storage::storage::Storage;
use crate::core::commands::invoker::Invoker;


fn main() {

    let db_path = String::from("src/database.db");
    let mut invoker = Invoker::new(db_path);
    invoker.init();

    let mut cli = Cli::new(invoker);
    cli.run();

}
