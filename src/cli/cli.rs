#![allow(unused)]
use std::io;
use std::io::Write;
use crate::core::commands::invoker::Invoker;
use crate::cli::utils::parse_user_input;
use crate::core::auth::entities::user::User;

pub struct Cli {
    user_input: String,
    invoker: Invoker,
    user: User
}


impl Cli {
    pub fn new(invoker: Invoker, auth_user: User) -> Self {
        Self { 
            user_input: String::new(),
            invoker,
            user: auth_user
        }
    }

    pub fn run(&mut self) {
        loop {
            self.read_line();
            if (self.get_user_input().trim().eq_ignore_ascii_case("!q")) {
                println!("Bye!");
                break;
            }

            let mut payload = parse_user_input(&self.get_user_input());
            payload.set_auth_user(&self.user);
            let result = self.invoker.execute_command(payload);
            let data = match result {
                Ok(data) => data,
                Err(err) => {
                    println!("{}", err);
                    continue;
                }
            };

            // for operations that doesn't retrieve data from DB
            if data.get_data().len() == 0 {
                println!("{:}", data.get_message());
                continue;
            }

            println!("{:}", data.get_message());
            println!("{:}", data.get_data());
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
