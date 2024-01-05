#![allow(unused)]
mod cli;
mod storage;
mod core;

use std::process;

use crate::cli::cli::Cli;
use crate::storage::storage::Storage;
use crate::core::commands::invoker::Invoker;

use crate::core::auth::utils::auth;

use crate::core::utils::file_io::{get_cypher_key_path, get_file_data};

fn main() {

    // println!("{}", get_cypher_key_path().unwrap());
    // println!("{}", get_file_data(&get_cypher_key_path().unwrap()).unwrap());

    let db_path = String::from("src/database.db");

    match auth(&db_path) {
        Ok(message) => println!("{}", message),
        Err(err) => {
            println!("{}", err.to_string());
            process::exit(0);
        }
    }

    let mut invoker = Invoker::new(db_path);
    invoker.init();

    let mut cli = Cli::new(invoker);
    cli.run();

}
