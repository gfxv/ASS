
use std::io::{self, Write};

pub struct Prompt;

impl Prompt {
    pub fn new(inquery_text: &String) -> Result<String, io::Error> {
        
        let mut answer = String::new();

        print!("{:}", inquery_text);
        io::stdout().flush();

        match io::stdin().read_line(&mut answer) {
            Ok(n) => return Ok(answer.trim().to_string()),
            Err(error) => return Err(error),
        }
    }
}
