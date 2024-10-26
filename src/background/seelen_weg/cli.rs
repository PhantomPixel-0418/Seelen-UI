use clap::Command;

use crate::{error_handler::Result, get_subcommands};

use super::SeelenWeg;

get_subcommands![
    /** Open Dev Tools (only works if the app is running in dev mode) */
    Debug,
];

impl SeelenWeg {
    pub const CLI_IDENTIFIER: &'static str = "weg";

    pub fn get_cli() -> Command {
        Command::new(Self::CLI_IDENTIFIER)
            .about("Seelen's Weg")
            .arg_required_else_help(true)
            .subcommands(SubCommand::commands())
    }

    pub fn process(&mut self, matches: &clap::ArgMatches) -> Result<()> {
        let subcommand = SubCommand::try_from(matches)?;
        match subcommand {
            SubCommand::Debug => {
                #[cfg(any(debug_assertions, feature = "devtools"))]
                self.window.open_devtools();
            }
        };
        Ok(())
    }
}
