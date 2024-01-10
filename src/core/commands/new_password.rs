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

// usage:
// new
//     [promt] name:
//     [promt] password: 
//     -> status, message

pub struct NewPasswordCommand {
    name: String,
    desc: String
}

impl NewPasswordCommand {
    pub fn new(name: String, desc: String) -> Self {
        NewPasswordCommand {name, desc}
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

impl Command for NewPasswordCommand {

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_description(&self) -> String {
        self.desc.to_string()
    }

    fn execute(&self, data: CommandPayload) -> Result<ReturnData, String> {

        bearer::mod_access(&data.get_user())?;

        let mut name = data.get_arg().to_string();

        if name.is_empty() {
            name = Prompt::new(&String::from("Resource name: "))
                .map_err(|err| format!("[CORE.ERROR] Can't read user's `Resource name` input\n{}", err.to_string()))?;
        }

        let mut password = Prompt::new(&String::from("Value: "))
            .expect("[CORE.ERROR] Can't read user's `Password` input");

        match analyze_password(&password) {
            Ok(_) => true,
            Err(err) => {
                println!("{}", err.to_string());
                if self.handle_err_on_analyze() {
                    password = generate_password();
                };
                true
            },
        };

        let password_crud = Storage::new(
            data.get_path().to_owned()
        ).get_password_crud();

        let encrypted_name = encrypt_data(&name)?;
        let encrypted_password = encrypt_data(&password)?;
        password_crud.insert_new_password(&encrypted_name, &encrypted_password)
    }

}


