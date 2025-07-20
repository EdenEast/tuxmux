use crate::{cmd::cli::List, config::Config, mux::Mux, walker::Walker};

use super::Run;

impl Run for List {
    fn run(self) -> miette::Result<()> {
        let config = Config::load()?;
        if self.all {
            for path in config.paths_from_walk() {
                println!("{}", path);
            }

            return Ok(());
        }

        let names = config.mux.list_sessions();
        let max_name = names.iter().map(|s| s.len()).max().unwrap_or_default();

        for s in names {
            println!("{:npad$}", s, npad = max_name);
        }

        Ok(())
    }
}
