
use std::io::{self, Write};

pub struct HiddenPrompt;

impl HiddenPrompt {
    pub fn new(inquery_text: &String) -> Result<String, io::Error> {
        let answer = rpassword::prompt_password(inquery_text);
        return match answer {
            Ok(n) => Ok(n.trim().to_string()),
            Err(error) => Err(error),
        }
    }
}
