
use crate::core::entities::cmd_data::CommandData;

pub trait Command {

    fn get_name(&self) -> String;

    fn get_description(&self) -> String;

    fn execute(&self, cmd_data: CommandData);
}