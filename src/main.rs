#![allow(unused)]
mod cli;
mod storage;

use crate::cli::cli::Cli;
use crate::storage::storage::Storage;

use std::io;

fn main() {

    const DB_PATH: &str = "./storage/database.db";
    
    let db = Storage::new(DB_PATH.to_string());
        
    let mut cli = Cli::new();
    cli.run();

}
