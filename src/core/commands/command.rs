

pub trait Command {

    fn get_name(&self) -> String;

    fn get_description(&self) -> String;

    fn execute(&self);
}