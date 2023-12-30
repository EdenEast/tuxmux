use clap::CommandFactory;

use super::{Cli, Completion, Run};

impl Run for Completion {
    fn run(self) -> miette::Result<()> {
        let mut cmd = Cli::command();
        let name = cmd.get_name().to_string();
        clap_complete::generate(self.generator, &mut cmd, name, &mut std::io::stdout());
        Ok(())
    }
}
