
use crate::{
    core::{
        commands::command::Command,
        entities::command_payload::CommandPayload,
        entities::return_data::ReturnData
    },
    storage::storage::Storage
};
use crate::core::entities::prompt::Prompt;
use crate::core::security::crypto::encrypt_data;


pub struct UpdatePasswordCommand {
    name: String,
    desc: String
}

impl UpdatePasswordCommand {
    pub fn new(name: String, desc: String) -> Self {
        UpdatePasswordCommand {name, desc}
    }
}

impl Command for UpdatePasswordCommand {

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_description(&self) -> String {
        self.desc.to_string()
    }

    fn execute(&self, data: CommandPayload) -> Result<ReturnData, String> {
        let raw_name = data.get_arg();
        let raw_new_password = Prompt::new(&String::from("New Password: "))
            .map_err(|err| format!("[CORE.ERROR] Can't read user's `Password` input\n{}", err.to_string()))?;

        let password_crud = Storage::new(
            data.get_path().to_owned()
        ).get_password_crud();

        let name = encrypt_data(raw_name)?;
        let new_password = encrypt_data(&raw_new_password)?;

        password_crud.update_by_name(&name, &new_password)
    }
}
