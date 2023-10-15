
use crate::core::commands::command::Command;

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

    fn execute(&self) {
        todo!()
    }
}


