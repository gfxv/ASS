
use crate::{core::{
    commands::command::Command,
    entities::cmd_data::CommandData,
    entities::return_data::ReturnData
}, storage::storage::Storage};
use crate::core::entities::prompt::Prompt;


pub struct CreateGroupCommand {
    name: String,
    desc: String
}

impl CreateGroupCommand {
    pub fn new(name: String, desc: String) -> Self {
        CreateGroupCommand {name, desc}
    }
}

impl Command for CreateGroupCommand {

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_description(&self) -> String {
        self.desc.to_string()
    }

    fn execute(&self, data: CommandData) -> ReturnData {
        let name = data.get_arg();
        let raw_access_level = Prompt::new(&String::from("Access Level: "))
            .expect("[CORE.ERROR] Can't read user's `Access Level` input");

        let access_level = raw_access_level.parse::<u16>()
            .expect("[CORE.ERROR] Can't convert user's `Access Level` input");

        let group_crud = Storage::new(
            data.get_path().to_owned()
        ).get_group_crud();

        group_crud.create_group(name, access_level)
    }
}


