
use crate::core::entities::{
    command_payload::CommandPayload,
    return_data::ReturnData
};

pub trait Command {

    fn get_name(&self) -> String;

    fn get_description(&self) -> String;

    fn execute(&self, cmd_data: CommandPayload) -> Result<ReturnData, String>;
}