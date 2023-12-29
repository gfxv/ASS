
use crate::{core::{
    commands::command::Command,
    entities::cmd_data::CommandData,
    entities::return_data::ReturnData
}, storage::storage::Storage};
use crate::core::entities::prompt::Prompt;

// usage:
// get <name>
//     -> status, message (TODO save password to clipboard)


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

    fn execute(&self, data: CommandData) -> ReturnData {
        let name = data.get_arg();
        let new_password = Prompt::new(&String::from("New Password: "))
            .expect("[CORE.ERROR] Can't read user's `Password` input");

        let password_crud = Storage::new(
            data.get_path().to_owned()
        ).get_password_crud();

        password_crud.update_by_name(&name, &new_password)
    }
}


