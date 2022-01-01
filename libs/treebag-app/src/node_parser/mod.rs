use std::rc::Rc;

use crate::WorksheetNode;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("IO error")]
    IoError(#[from] std::io::Error),
}

pub fn parse_worksheet_node() -> Result<Rc<dyn WorksheetNode>, ParseError> {
    log::trace!("parse_worksheet_node");
    unimplemented!("parse_worksheet_node")
} 

pub fn parse_worksheet_node_from_file(path: &str) -> Result<Rc<dyn WorksheetNode>, ParseError> {
    log::trace!("parse_worksheet_node_from_file");
    unimplemented!("parse_worksheet_node")
} 
