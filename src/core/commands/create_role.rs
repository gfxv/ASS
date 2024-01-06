
use crate::{core::{
    commands::command::Command,
    entities::command_payload::CommandPayload,
    entities::return_data::ReturnData
}, storage::storage::Storage};
use crate::core::entities::prompt::Prompt;
use crate::core::utils::bearer;


pub struct CreateRoleCommand {
    name: String,
    desc: String
}

impl CreateRoleCommand {
    pub fn new(name: String, desc: String) -> Self {
        CreateRoleCommand {name, desc}
    }
}

impl Command for CreateRoleCommand {

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_description(&self) -> String {
        self.desc.to_string()
    }

    fn execute(&self, data: CommandPayload) -> Result<ReturnData, String> {

        bearer::admin_only(data.get_user())?;

        let mut name = data.get_arg().to_string();

        if name.is_empty() {
            name = Prompt::new(&String::from("Role name: "))
                .map_err(|err| format!("[CORE.ERROR] Can't read user's `Role name` input\n{}", err.to_string()))?;
        }

        let raw_access_level = Prompt::new(&String::from("Access level: "))
            .map_err(|err| format!("[CORE.ERROR] Can't read user's `Access Level` input\n{}", err.to_string()))?;

        let access_level = raw_access_level.parse::<u16>()
            .map_err(|err| format!("[CORE.ERROR] Can't convert user's `Access Level` input\n{}", err.to_string()))?;

        let role_crud = Storage::new(
            data.get_path().to_owned()
        ).get_role_crud();

        role_crud.create_role(&name, access_level)
    }
}


