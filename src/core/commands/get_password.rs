
use crate::{core::{
    commands::command::Command, 
    entities::cmd_data::CommandData,
    entities::return_data::ReturnData
}, storage::storage::Storage};

// usage:
// get <name>
//     -> status, message (TODO save password to clipboard)


pub struct GetPasswordCommand {
    name: String,
    desc: String
}

impl GetPasswordCommand {
    pub fn new(name: String, desc: String) -> Self {
        GetPasswordCommand {name, desc}
    }
}

impl Command for GetPasswordCommand {

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_description(&self) -> String {
        self.desc.to_string()
    }

    fn execute(&self, data: CommandData) -> ReturnData {
        let name = data.get_arg();
        if !(name.len() > 0) {
            println!("bad input");
            return ReturnData::new(String::from(""), 2, String::from(""));
        }

        let password_crud = Storage::new(
            data.get_path().to_owned()
        ).get_password_crud();

        password_crud.get_password_by_name(name)
    }
}


