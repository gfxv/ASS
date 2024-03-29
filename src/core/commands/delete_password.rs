
use crate::core::{
    commands::command::Command,
    entities::{
        prompt::Prompt,
        command_payload::CommandPayload,
        return_data:: ReturnData
    }
};
use crate::core::utils::bearer;
use crate::storage::storage::Storage;


pub struct DeletePasswordCommand {
    name: String,
    desc: String
}

impl DeletePasswordCommand {
    pub fn new(name: String, desc: String) -> Self {
        DeletePasswordCommand {name, desc}
    }
}

impl Command for DeletePasswordCommand {

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_description(&self) -> String {
        self.desc.to_string()
    }

    fn execute(&self, data: CommandPayload) -> Result<ReturnData, String> {

        bearer::admin_access(&data.get_user());

        let name = data.get_arg();

        let password_crud = Storage::new(
            data.get_path().to_owned()
        ).get_password_crud();

        password_crud.delete_password_by_name(&name)
    }

}


