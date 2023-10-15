#![allow(unused)]
use std::io;


pub struct Cli {
    user_input: String,
}


impl Cli {
    pub fn new() -> Self {
        Self { 
            user_input: String::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            self.read_line();
            if (self.get_user_input().trim().eq_ignore_ascii_case("!q")) {
                println!("Bye!");
                break;
            }
            println!("Your input: {}", self.get_user_input());
        }
    }

    pub fn read_line(&mut self) {
        self.user_input.clear();
        io::stdin()
            .read_line(&mut self.user_input)
            .expect("failed to read user input (in cli)");
    }
        
    pub fn get_user_input(&mut self) -> String {
        self.user_input.to_owned()
    }

} 
