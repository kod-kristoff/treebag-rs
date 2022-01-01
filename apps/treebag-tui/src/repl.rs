use crate::MetaCommand;
use crate::TreebagCommand;

#[derive(Debug, PartialEq)]
pub enum CommandType {
    MetaCommand(MetaCommand),
    TreebagCommand(TreebagCommand),
}

pub fn get_command_type(command: &String) -> CommandType {
    match command.starts_with(".") {
        true => CommandType::MetaCommand(MetaCommand::new(command.to_owned())),
        false => CommandType::TreebagCommand(TreebagCommand::new(command.to_owned())),
    }
}
