use std::alloc::handle_alloc_error;
use crate::core::{
    commands::command::Command,
    entities::{
        prompt::Prompt,
        command_payload::CommandPayload,
        return_data:: ReturnData
    },
    security::utils::{analyze_password, generate_password},
    security::crypto::{encrypt_data, decrypt_data}
};
use crate::core::utils::bearer;
use crate::storage::storage::Storage;

pub struct ClearTerminalCommand {
    name: String,
    desc: String
}

impl ClearTerminalCommand {
    pub fn new(name: String, desc: String) -> Self {
        ClearTerminalCommand {name, desc}
    }

    // probably name is a bit wierd, but nothing better came to my mind at 4 a.m.
    fn handle_err_on_analyze(&self) -> bool {
        let answer = Prompt::new(&String::from("[SEC.INFO] Wanna use generated password instead (y/n)? ")).unwrap().to_lowercase();
        return match answer.trim() {
            "y" => true,
            "n" => false,
            _ => false
        }
    }
}

impl Command for ClearTerminalCommand {

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_description(&self) -> String {
        self.desc.to_string()
    }

    fn execute(&self, data: CommandPayload) -> Result<ReturnData, String> {

        std::process::Command::new("clear").status().unwrap();

        Ok(ReturnData::new(String::from("Terminal cleared."), 1, String::new()))


    }

}


