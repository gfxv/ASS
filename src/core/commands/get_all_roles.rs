
use crate::{core::{
    commands::command::Command,
    entities::command_payload::CommandPayload,
    entities::return_data::ReturnData
}, storage::storage::Storage};
use crate::core::utils::bearer;

// usage:
// get <name>
//     -> status, message (TODO save password to clipboard)


pub struct GetAllGroupsCommand {
    name: String,
    desc: String
}

impl GetAllGroupsCommand {
    pub fn new(name: String, desc: String) -> Self {
        GetAllGroupsCommand {name, desc}
    }
}

impl Command for GetAllGroupsCommand {

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_description(&self) -> String {
        self.desc.to_string()
    }

    fn execute(&self, data: CommandPayload) -> Result<ReturnData, String> {

        bearer::admin_only(&data.get_user())?;

        let role_crud = Storage::new(
            data.get_path().to_owned()
        ).get_role_crud();

        role_crud.get_all_roles()
    }
}


