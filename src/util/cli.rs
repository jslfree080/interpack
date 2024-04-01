use clap::{ArgMatches, Command};

pub struct Builder {
    matches: ArgMatches,
}

impl Builder {
    pub fn new(cmd_param: Command) -> Self {
        Self {
            matches: cmd_param.get_matches(),
        }
    }

    pub fn get_matches(&self) -> &ArgMatches {
        &self.matches
    }
}
