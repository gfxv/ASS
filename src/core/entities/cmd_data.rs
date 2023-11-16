

// Structure of CommandArgument will be changed.
// Current implementation is just for testing.

pub struct CommandData {
    db_path: String,
    command_name: String,
    arg: String
}

impl CommandData {

    pub fn new(command_name: String, arg: String) -> Self {
        Self {
            db_path: String::new(),
            command_name,
            arg
        }
    }

    pub fn set_path(&mut self, path: &String) {
        self.db_path = path.to_string();
    }

    pub fn get_cmd(&self) -> &String {
        &self.command_name
    }

    pub fn get_arg(self) -> String {
        self.arg
    }
}