
#[derive(Debug, PartialEq)]
pub enum CommandType {
    MetaCommand(String),
}

pub fn get_command_type(command: &str) -> CommandType {
    match command.starts_with(".") {
        true => CommandType::MetaCommand(command.to_string()),
        false => unimplemented!(),
    }
}
