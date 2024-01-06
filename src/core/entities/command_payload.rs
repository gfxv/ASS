use std::ptr::null;
use crate::core::auth::entities::user::User;

pub struct CommandPayload {
    db_path: String,
    command_name: String,
    arg: String,
    user: Option<User>
}

impl CommandPayload {

    pub fn new(command_name: String, arg: String) -> Self {
        Self {
            db_path: String::new(),
            command_name,
            arg,
            user: None
        }
    }

    pub fn set_path(&mut self, path: &String) {
        self.db_path = path.to_string();
    }

    pub fn set_auth_user(&mut self, user: &User) {
        self.user = Some(user.clone());
    }

    // When you `get_user` in <command>.execute user is always set, so no need for to check for `None` value
    pub fn get_user(&self) -> &User {
        &self.user.unwrap()
    }

    pub fn get_path(&self) -> &String {
        &self.db_path
    }   

    pub fn get_cmd(&self) -> &String {
        &self.command_name
    }

    pub fn get_arg(&self) -> &String {
        &self.arg
    }
}