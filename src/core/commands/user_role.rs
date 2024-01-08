
use crate::core::{
    commands::command::Command,
    entities::{
        prompt::Prompt,
        command_payload::CommandPayload,
        return_data::ReturnData,
    },
    utils::bearer,
};
use crate::storage::storage::Storage;

pub struct UserRoleCommand {
    name: String,
    desc: String
}

impl UserRoleCommand {
    pub fn new(name: String, desc: String) -> Self {
        UserRoleCommand {name, desc}
    }
}

impl Command for UserRoleCommand {

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_description(&self) -> String {
        self.desc.to_string()
    }

    fn execute(&self, data: CommandPayload) -> Result<ReturnData, String> {

        bearer::admin_only(&data.get_user())?;

        let mut user_name = data.get_arg().to_string();
        if user_name.is_empty() {
            user_name = Prompt::new(&String::from("Username: ")).expect("Can't read user's input");
        }

        let storage = Storage::new(data.get_path().to_string());
        let role_crud = storage.get_role_crud();
        let auth_crud = storage.get_auth_crud();

        // [1]
        let user_exists = auth_crud.user_already_exists_by_username(&user_name)?;
        if !user_exists {
            return Err(String::from(format!("[CORE.ERROR] No such user with name `{}`", user_name)));
        }

        let user_id = auth_crud.get_user_id_by_name(&user_name)?; // [2]
        let user_roles = role_crud.get_users_roles(user_id)?; // [3]
        let all_roles = role_crud.get_all_roles()?; // [4]

        println!();
        println!("Current roles:");
        user_roles.iter().for_each(|role| {
            println!("[{}] {} (access level = {})", role.get_id(), role.get_name(), role.get_access_level());
        });

        println!();

        println!("Available roles:");
        println!("{}\n", all_roles.get_data());

        let roles_to_add_raw = Prompt::new(&String::from("What roles would you like to ASSIGN (type numbers separated by commas), press ENTER to skip: ")).expect("Can't read user's input");
        let mut roles_to_add: Vec<u16> = Vec::new();
        if !roles_to_add_raw.is_empty() {
            roles_to_add = roles_to_add_raw
                .split(",")
                .into_iter()
                .map(|role| {
                    role.trim().parse::<u16>().expect("Can't cast user's role to u16")
                }).collect();
        }

        let roles_to_remove_raw = Prompt::new(&String::from("What roles would you like to REMOVE (type numbers separated by commas), press ENTER to skip: ")).expect("Can't read user's input");
        let mut roles_to_remove: Vec<u16> = Vec::new();
        if !roles_to_remove_raw.is_empty() {
            roles_to_remove = roles_to_remove_raw
                .split(",")
                .into_iter()
                .map(|role| {
                    role.trim().parse::<u16>().expect("Can't cast user's role to u16")
                }).collect();
        }

        roles_to_add.iter().for_each(|&role| {
            role_crud.add_role(user_id, role);
        });

        roles_to_remove.iter().for_each(|&role| {
            role_crud.remove_role(user_id, role);
        });

        Ok(ReturnData::new(String::new(), 1, String::new()))
    }

}


