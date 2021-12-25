use std::fmt;

use crate::{AppError, Response};


#[derive(Debug, PartialEq)]
pub enum MetaCommand {
    Exit,
    Help,
    Open(String),
    Unknown,
}

/// Trait responsible for translating type into a formated text.
impl fmt::Display for MetaCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MetaCommand::Exit => f.write_str(".exit"),
            MetaCommand::Help => f.write_str(".help"),
            MetaCommand::Open(_) => f.write_str(".open"),
            MetaCommand::Unknown => f.write_str("Unknown command"),
        }
    }
}

impl MetaCommand {
    pub fn new(command: String) -> MetaCommand {
        let args: Vec<&str> = command.split_whitespace().collect();
        let cmd = args[0].to_owned();
        match cmd.as_ref() {
            ".exit" => MetaCommand::Exit,
            ".help" => MetaCommand::Help,
            ".open" => MetaCommand::Open(command),
            _ => MetaCommand::Unknown,
        }
    }
}

pub fn handle_meta_command(command: MetaCommand) -> Response<String> {
    match command {
        MetaCommand::Exit => Response::Exit,
        MetaCommand::Help => Response::Ok(format!(
            "{}{}{}{}{}{}{}{}{}",
            "Special commands:\n",
            ".help            - Display this message\n",
            ".new             - Close existing worksheet and create empty\n",
            ".open <FILENAME> - Close existing worksheet and reopen FILENAME\n",
            ".save <FILENAME> - Write in-memory worksheet into FILENAME\n",
            ".add <FILENAME>  - Read input from FILENAME\n",
            ".graph           - List names of the component graph\n",
            ".ast <QUERY>     - Show the abstract syntax tree for QUERY.\n",
            ".exit            - Quits this application"
        )),
        MetaCommand::Open(args) => Response::Ok(format!("To be implemented: {}", args)),
        MetaCommand::Unknown => Response::Err(AppError::UnknownCommand(format!(
            "Unknown command or invalid arguments. Enter '.help'"
        ))),
    }
}
