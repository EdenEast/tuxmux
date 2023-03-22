use clap::CommandFactory;

use crate::cli::{Cli, Completion};

use super::ExecuteableCmd;

impl ExecuteableCmd for Completion {
    fn execute(self) -> eyre::Result<()> {
        let mut cmd = Cli::command();
        let name = cmd.get_name().to_string();
        clap_complete::generate(self.genreator, &mut cmd, name, &mut std::io::stdout());
        Ok(())
    }
}
