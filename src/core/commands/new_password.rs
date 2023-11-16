
use crate::core::{
    commands::command::Command, 
    entities::prompt::Prompt
};

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
        let name = Prompt::new(&String::from("Resource name: "))
            .expect("[CORE.ERROR] Can't read user's `Resource name` input");
        let password = Prompt::new(&String::from("Password: "))
            .expect("[CORE.ERROR] Can't read user's `Password` input");

        println!("{}:{}", name, password);
    }

}


