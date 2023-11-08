

// Structure of CommandArgument will be changed.
// Current implementation is just for testing.
pub struct CommandArgument {
    command_name: String,
    arg: String
}

impl CommandArgument {

    pub fn new(command_name: String, arg: String) -> Self {
        Self {
            command_name,
            arg
        }
    }

    pub fn get_cmd(self) -> String {
        self.command_name
    }

    pub fn get_arg(self) -> String {
        self.arg
    }
}