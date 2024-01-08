#![allow(unused)]
mod cli;
mod storage;
mod core;

use std::{
    process, env
};

use crate::cli::cli::Cli;
use crate::core::commands::invoker::Invoker;
use crate::core::auth::utils::auth;
use crate::core::utils::initializer;

fn main() {

    let db_path = String::from("src/database.db");

    let args: Vec<String> = env::args().collect();
    if (args.contains(&String::from("init"))) {
        initializer::initialize(&db_path).expect("Error occurred while initializing app :(");
    }

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
