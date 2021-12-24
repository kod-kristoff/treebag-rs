use std::env;
mod worksheet;
mod component_graph;
mod node;

use crate::{
    worksheet::Worksheet,
};

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .format_timestamp(None)
        .init();
    log::trace!("called main");
    let args: Vec<String> = env::args().collect();
    log::debug!("args = {:?}", args);
    let worksheet = if args.len() == 0 { 
        // Worksheet::new(None, None)
        Worksheet::new()
    } else {
        // Worksheet::new(None, Some(args[0]))
        Worksheet::new()
    };
}
