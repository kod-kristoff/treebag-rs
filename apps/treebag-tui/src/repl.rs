use crate::MetaCommand;

#[derive(Debug, PartialEq)]
pub enum CommandType {
    MetaCommand(MetaCommand),
}

pub fn get_command_type(command: &String) -> CommandType {
    match command.starts_with(".") {
        true => CommandType::MetaCommand(MetaCommand::new(command.to_owned())),
        false => unimplemented!(),
    }
}
