mod error;
mod meta_command;
mod repl;
mod treebag_command;

use rustyline::{
    error::ReadlineError,
    Editor,
};

use repl::{CommandType, get_command_type};

use treebag_app::{Worksheet};

pub use error::{AppError, Response, Result};
pub use crate::meta_command::MetaCommand;
pub use crate::treebag_command::TreebagCommand;
use crate::meta_command::{handle_meta_command};
use crate::treebag_command::handle_treebag_command;


fn main() -> rustyline::Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .format_timestamp(None)
        .init();
    log::trace!("treebag_tui::main");

    let mut repl = Editor::<()>::new();
    if repl.load_history("history").is_err() {
        log::warn!("No previous history");
    }

    let mut count = 1;

    let mut worksheet = Worksheet::new();
    loop {
        if count == 1 {
            // Friendly intro message for the user
            println!("{}{}{}{}{}",
            "treebag-tui - 0.1.0\n",
            "Enter .exit to quit.\n",
            "Enter .help for usage hints.\n",
            "Connected to a transient in-memory worksheet.\n",
            "Use '.open FILENAME' to reopen on a persistent worksheet.");
            //TODO: Get info about application name and version dinamically.
        }

        let p = format!("treebag | {}> ", count);
        //repl.helper_mut()
        //    .expect("No helper found")
        //    .colored_prompt = format!("\x1b[1;32m{}\x1b[0m", p);
        // Source for ANSI Color information: http://www.perpetualpc.net/6429_colors.html#color_list
        // http://bixense.com/clicolors/

        let readline = repl.readline(&p);
        match readline {
            Ok(command) => {
                repl.add_history_entry(command.as_str());
                match get_command_type(&command.trim().to_owned()) {
                    CommandType::MetaCommand(cmd) => {
                        log::debug!("MetaCommand = {:?}", cmd);
                        let _ = match handle_meta_command(cmd) {
                            Response::Exit => break,
                            Response::Ok(response) => println!("{}", response),
                            Response::Err(err) => log::error!("Error: {}", err),
                        };
                    },
                    CommandType::TreebagCommand(cmd) => {
                        log::debug!("TreebagCommand = {:?}", cmd);
                        let _ = match handle_treebag_command(cmd, &mut worksheet) {
                            Ok(response) => println!("{}", response),
                            Err(err) => log::error!("Error: {}", err)
                        };
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
        count += 1;
    }
    repl.append_history("history").unwrap();
    
    Ok(())
}
