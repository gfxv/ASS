#![allow(unused)]
mod cli;
mod storage;
mod core;

use crate::cli::cli::Cli;
use crate::storage::storage::Storage;
use crate::core::commands::invoker::Invoker;


fn main() {

    let db_path = String::from("./storage/database.db");
    // let db = Storage::new(DB_PATH.to_string());
    let mut invoker = Invoker::new(db_path);
    invoker.init();

    let mut cli = Cli::new(invoker);
    cli.run();

}
