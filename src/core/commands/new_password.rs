
use crate::core::commands::command::Command;

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

    fn execute(&self) {
        todo!()
    }

}


