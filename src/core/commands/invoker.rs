
use std::collections::HashMap;
use crate::core::commands::command::Command;
use crate::core::commands::*;
use crate::core::entities::command_payload::CommandPayload;
use crate::storage::storage::Storage;
use crate::core::entities::return_data::ReturnData;


pub struct Invoker {
    storage: String,
    commands: HashMap<String, Box<dyn Command>>
}

// TODO add description to commands

impl Invoker {

    pub fn new(storage: String) -> Self {
        let commands = HashMap::new();
        Self { 
            storage, 
            commands
        }
    }

    pub fn init(&mut self) {

        //----------------------//
        //  Password Commands   //
        //----------------------//

        let get_password_command = get_password::GetPasswordCommand::new(
            "get".to_string(), "get resource's password".to_string()
        );
        self.commands.insert(
            get_password_command.get_name(), 
            Box::new(get_password_command)
        );

        let new_password_command = new_password::NewPasswordCommand::new(
            "new".to_string(), "create new resource".to_string()
        );
        self.commands.insert(
            new_password_command.get_name(), 
            Box::new(new_password_command)
        );

        let update_password_command = update_password::UpdatePasswordCommand::new(
            "update".to_string(), "update resource's password".to_string()
        );
        self.commands.insert(
            update_password_command.get_name(),
            Box::new(update_password_command)
        );

        let delete_password_command = delete_password::DeletePasswordCommand::new(
          "delete".to_string(), "delete resource".to_string()
        );
        self.commands.insert(
            delete_password_command.get_name(),
            Box::new(delete_password_command)
        );

        //-------------------//
        //  Group Commands   //
        //-------------------//

        // ng = new group
        let create_group_command = create_group::CreateGroupCommand::new(
            "ng".to_string(), "create new group".to_string()
        );
        self.commands.insert(
            create_group_command.get_name(),
            Box::new(create_group_command)
        );

        let group_command = password_group::PasswordGroupCommand::new(
            "group".to_string(), "add/remove password to/from group(s)".to_string()
        );
        self.commands.insert(
            group_command.get_name(),
            Box::new(group_command)
        );

        let get_all_groups_command = get_all_groups::GetAllGroupsCommand::new(
          "groups".to_string(), "list all available groups".to_string()
        );
        self.commands.insert(
            get_all_groups_command.get_name(),
            Box::new(get_all_groups_command)
        );

        //------------------//
        //  Role Commands   //
        //------------------//

        let create_role_command = create_role::CreateRoleCommand::new(
            "nr".to_string(), "create new role".to_string()
        );
        self.commands.insert(
            create_role_command.get_name(),
            Box::new(create_role_command)
        );

        let role_command = user_role::UserRoleCommand::new(
            "role".to_string(), "add/remove user to/from role(s)".to_string()
        );
        self.commands.insert(
            role_command.get_name(),
            Box::new(role_command)
        );

        let get_all_roles_command = get_all_roles::GetAllGroupsCommand::new(
            "roles".to_string(), "list all available roles".to_string()
        );
        self.commands.insert(
            get_all_roles_command.get_name(),
            Box::new(get_all_roles_command)
        );

        let clear = clear_terminal::ClearTerminalCommand::new(
            "clear".to_string(), "clears terminal".to_string()
        );
        self.commands.insert(
            clear.get_name(),
            Box::new(clear)
        );

    }


    pub fn execute_command(&self, mut data: CommandPayload) -> Result<ReturnData, String> {
        if let Some(command) = self.commands.get(data.get_cmd()) {
            data.set_path(&self.storage);
            command.execute(data)
        } else {
            Ok(ReturnData::new(String::from("Command '") + data.get_cmd() + "' not found.", 4, String::from("")))
        }
    }
    
}


