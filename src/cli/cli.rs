#![allow(unused)]
use std::io;
use std::io::Write;
use crate::core::commands::invoker::Invoker;
use crate::cli::utils::parse_user_input;

pub struct Cli {
    user_input: String,
    invoker: Invoker
}


impl Cli {
    pub fn new(invoker: Invoker) -> Self {
        Self { 
            user_input: String::new(),
            invoker
        }
    }

    pub fn run(&mut self) {
        loop {
            self.read_line();
            if (self.get_user_input().trim().eq_ignore_ascii_case("!q")) {
                println!("Bye!");
                break;
            }

            let cmd_data = parse_user_input(&self.get_user_input());
            
            let result = self.invoker.execute_command(cmd_data);

            if result.get_status().to_owned() != 1 {
                println!("{:}", result.get_message());
                continue;
            }

            // for operations that doesn't retreive data from DB
            if result.get_data().len() == 0 {
                println!("{:}", result.get_message());
                continue;
            }

            println!("{:}", result.get_data());
        }
    }

    fn read_line(&mut self) {
        self.user_input.clear();
        io::stdin()
            .read_line(&mut self.user_input)
            .expect("failed to read user input (in cli)");
    }
        
    fn get_user_input(&mut self) -> String {
        self.user_input.to_owned()
    }

} 
