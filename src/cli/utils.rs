
use crate::core::entities::cmd_data::CommandData;

pub fn parse_user_input(input: &String) -> CommandData {
    let parts = input.split(" ").collect::<Vec<_>>();
    let cmd = parts[0].trim().to_string();
    let mut arg = String::from("");
    if parts.len() == 2 {
        arg = parts[1].to_string();
    }
    let cmd_data = CommandData::new(cmd, arg);
    
    cmd_data
}