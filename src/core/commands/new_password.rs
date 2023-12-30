
use crate::core::{
    commands::command::Command, 
    entities::{
        prompt::Prompt, 
        cmd_data::CommandData,
        return_data:: ReturnData
    }
};
use crate::storage::storage::Storage;

// usage:
// new
//     [promt] name:
//     [promt] password: 
//     -> status, message

pub struct NewPasswordCommand {
    name: String,
    desc: String
}

impl NewPasswordCommand {
    pub fn new(name: String, desc: String) -> Self {
        NewPasswordCommand {name, desc}
    }
}

impl Command for NewPasswordCommand {

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_description(&self) -> String {
        self.desc.to_string()
    }

    fn execute(&self, data: CommandData) -> Result<ReturnData, String> {
        let name = Prompt::new(&String::from("Resource name: "))
            .expect("[CORE.ERROR] Can't read user's `Resource name` input");
        let password = Prompt::new(&String::from("Password: "))
            .expect("[CORE.ERROR] Can't read user's `Password` input");

        let password_crud = Storage::new(
            data.get_path().to_owned()
        ).get_password_crud();
        
        password_crud.insert_new_password(&name, &password)
    }

}


