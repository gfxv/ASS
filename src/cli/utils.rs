
use crate::core::entities::cmd_data::CommandData;

pub fn parse_user_input(input: &String) -> CommandData {

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

    let cmd_data = CommandData::new(cmd, arg);

    cmd_data
}

