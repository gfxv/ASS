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

    let db_path = String::from("src/database.db");

    let user = match auth(&db_path) {
        Ok(user) => {
            println!("You logged in as {}", user.get_username());
            user
        },
        Err(err) => {
            println!("{}", err.to_string());
            process::exit(0);
        }
    };

    let mut invoker = Invoker::new(db_path);
    invoker.init();

    let mut cli = Cli::new(invoker, user);
    cli.run();

}
