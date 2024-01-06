
use crate::core::entities::command_payload::CommandPayload;

pub fn parse_user_input(input: &String) -> CommandPayload {

    let parts = input.split_once(" ");
    let mut cmd = input.trim().to_string();
    let mut arg = String::from("");
    if !parts.is_none() {
        cmd = parts.unwrap().0
            .to_string()
            .trim()
            .to_string();

        arg = parts.unwrap().1
            .to_string()
            .trim()
            .to_string();
    }

    let cmd_data = CommandPayload::new(cmd, arg);

    cmd_data
}

