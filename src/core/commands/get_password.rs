
use crate::{core::{
    commands::command::Command,
    entities::command_payload::CommandPayload,
    entities::return_data::ReturnData
}, storage::storage::Storage};
use crate::core::security::crypto::{decrypt_data, encrypt_data};
use crate::core::utils::bearer;

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

    fn execute(&self, data: CommandPayload) -> Result<ReturnData, String> {
        let raw_name = data.get_arg();
        if !(raw_name.len() > 0) {
            println!("bad input");
            return Ok(ReturnData::new(String::from(""), 2, String::from("")));
        }

        let storage = Storage::new(data.get_path().to_string());
        let password_crud = storage.get_password_crud();
        let group_crud = storage.get_group_crud();

        let name = encrypt_data(raw_name)?;

        bearer::default(data.get_path(), &data.get_user(), &name)?;

        let data = password_crud.get_password_by_name(&name)?;
        if data.get_data().is_empty() {
            return Ok(ReturnData::new(String::from("Password not found"), data.get_status().to_owned(), data.get_message().to_string()));
        }

        let decrypted = decrypt_data(data.get_data())?;
        Ok(ReturnData::new(decrypted, data.get_status().to_owned(), data.get_message().to_string()))
    }
}


