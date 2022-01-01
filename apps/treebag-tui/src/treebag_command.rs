
use treebag_app::{
    node_parser,
    Worksheet
};

use crate::Result;

#[derive(Debug, PartialEq)]
pub enum TreebagCommand {
    Component(ComponentCommand),
    Unknown(String),
}

impl TreebagCommand {
    pub fn new(command: String) -> Self {
        let v = command.split(" ").collect::<Vec<&str>>();
        match v[0] {
            "comp" => TreebagCommand::Component(ComponentCommand::new(command)),
            _ => TreebagCommand::Unknown(command),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ComponentCommand {
    Add(String),
    Create(String),
    Unknown(String),
}

impl ComponentCommand {
    pub fn new(command: String) -> Self {
        let v = command.split(" ").collect::<Vec<&str>>();
        match v[1] {
            "add" => ComponentCommand::Add(command),
            "create" => ComponentCommand::Create(command),
            _ => ComponentCommand::Unknown(command),
        }
    }
}

pub fn handle_treebag_command(
    command: TreebagCommand,
    worksheet: &mut Worksheet
) -> Result<String> {
    log::trace!("handle_treebag_command(command={:?})", command);

    match command {
        TreebagCommand::Component(cmd) => {
            match cmd {
                ComponentCommand::Add(cmd) => {
                    match node_parser::parse_worksheet_node_from_file(&cmd) {
                        Ok(node) => {
                            worksheet.graph_mut().add_node(node);
                            Ok(String::from("node added"))
                        },
                        Err(err) => {
                            log::error!("Parse error: {}", err);
                            unimplemented!()
                        }
                    }
                },
            _ => unimplemented!(),
            }
        },
        _ => unimplemented!()

    }
}
