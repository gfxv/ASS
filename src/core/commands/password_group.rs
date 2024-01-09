use crate::core::{
    commands::command::Command,
    entities::{
        prompt::Prompt,
        command_payload::CommandPayload,
        return_data::ReturnData,
    },
    utils::bearer,
};
use crate::core::security::crypto::encrypt_data;
use crate::storage::storage::Storage;

pub struct PasswordGroupCommand {
    name: String,
    desc: String
}

impl PasswordGroupCommand {
    pub fn new(name: String, desc: String) -> Self {
        PasswordGroupCommand {name, desc}
    }
}

impl Command for PasswordGroupCommand {

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_description(&self) -> String {
        self.desc.to_string()
    }

    fn execute(&self, data: CommandPayload) -> Result<ReturnData, String> {

        bearer::admin_access(&data.get_user())?;

        let mut resource_name = data.get_arg().to_string();
        if resource_name.is_empty() {
            resource_name = Prompt::new(&String::from("Resource name: ")).expect("Can't read user's input");
        }
        let encrypted_name = encrypt_data(&resource_name)?;

        let storage = Storage::new(data.get_path().to_string());
        let group_crud = storage.get_group_crud();
        let password_curd = storage.get_password_crud();

        let password_exists = password_curd.password_exists(&encrypted_name)?;
        if !password_exists {
            return Err(String::from(format!("[CORE.ERROR] No such resource with name `{}`", resource_name)));
        }

        let password_id = password_curd.get_password_id_by_name(&encrypted_name)?;
        let password_groups = group_crud.get_passwords_groups(password_id)?;
        let all_groups = group_crud.get_all_groups()?;

        println!();
        println!("Current groups:");
        password_groups.iter().for_each(|group| {
            println!("[{}] {} (access level = {})", group.get_id(), group.get_name(), group.get_access_level());
        });

        println!();

        println!("Available groups:");
        println!("{}\n", all_groups.get_data());

        let groups_to_add_raw = Prompt::new(&String::from("What groups would you like to ASSIGN (type numbers separated by commas), press ENTER to skip: ")).expect("Can't read user's input");
        let mut groups_to_add: Vec<u16> = Vec::new();
        if !groups_to_add_raw.is_empty() {
            groups_to_add = groups_to_add_raw
                .split(",")
                .into_iter()
                .map(|group| {
                    group.trim().parse::<u16>().map_err(|err| format!("[CORE.ERROR] Can't cast user's group {} to u16", group))
                }).collect::<Result<Vec<u16>,_>>()?;
        }

        let groups_to_remove_raw = Prompt::new(&String::from("What groups would you like to REMOVE (type numbers separated by commas), press ENTER to skip: ")).expect("Can't read user's input");
        let mut groups_to_remove: Vec<u16> = Vec::new();
        if !groups_to_remove_raw.is_empty() {
            groups_to_remove = groups_to_remove_raw
                .split(",")
                .into_iter()
                .map(|group| {
                    group.trim().parse::<u16>().map_err(|err| format!("[CORE.ERROR] Can't cast user's group {} to u16", group))
                }).collect::<Result<Vec<u16>,_>>()?;
        }

        groups_to_add.iter().for_each(|&group| {
            group_crud.add_group(password_id, group);
        });

        groups_to_remove.iter().for_each(|&group| {
            group_crud.remove_group(password_id, group);
        });

        Ok(ReturnData::new(String::from("Done!"), 1, String::new()))
    }

}
