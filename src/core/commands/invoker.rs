
use std::collections::HashMap;
use crate::core::commands::command::Command;
use crate::core::commands::*;
use crate::core::entities::cmd_data::CommandData;
use crate::storage::storage::Storage;
use crate::core::entities::return_data::ReturnData;


pub struct Invoker {
    storage: String,
    commands: HashMap<String, Box<dyn Command>>
}

// TODO add description to commands

impl Invoker {

    pub fn new(storage: String) -> Self {
        let commands = HashMap::new();
        Self { 
            storage, 
            commands
        }
    }

    pub fn init(&mut self) {

        let get_password_command = get_password::GetPasswordCommand::new(
            "get".to_string(), "LATER".to_string()
        );
        self.commands.insert(
            get_password_command.get_name(), 
            Box::new(get_password_command)
        );

        let new_password_command = new_password::NewPasswordCommand::new(
            "new".to_string(), "LATER".to_string()
        );
        self.commands.insert(
            new_password_command.get_name(), 
            Box::new(new_password_command)
        );
    }

    
    // remove println!, return object(struct) with status and message instead
    pub fn execute_command(&self, mut data: CommandData) -> ReturnData {
        if let Some(command) = self.commands.get(data.get_cmd()) {
            data.set_path(&self.storage);
            command.execute(data)
        } else {
            ReturnData::new(String::from("Command '") + data.get_cmd() + "' not found.", 4, String::from(""))
        }
    }
    
}


