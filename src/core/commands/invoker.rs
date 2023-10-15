
use std::collections::HashMap;
use crate::core::commands::command::Command;
use crate::core::commands::*;

pub struct Invoker {
    commands: HashMap<String, Box<dyn Command>>
}

// TODO add description to commands

impl Invoker {
    pub fn init(&mut self) {

        let get_password_command = get_password::GetPasswordCommand::new(
            "get".to_string(), "LATER".to_string()); 
        self.commands.insert(
            get_password_command.get_name(), 
            Box::new(get_password_command)
        );

        let new_password_command = new_password::NewPasswordCommand::new(
            "new".to_string(), "LATER".to_string());
        self.commands.insert(
            new_password_command.get_name(), 
            Box::new(new_password_command)
        );
    }

    
    // remove println!, return object(struct) with status and message instead
    pub fn execute_command(&self, name: &str) {
        if let Some(command) = self.commands.get(name) {
            command.execute();
        } else {
            println!("Command '{}' not found.", name); // return HERE
        }
    }
    
}


