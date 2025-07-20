use crate::{cmd::cli::Kill, config::Config, mux::Mux, ui::Picker};

use super::Run;

impl Run for Kill {
    fn run(self) -> miette::Result<()> {
        let config = Config::load()?;
        let names = config.mux.list_sessions();
        let query = self.query.as_ref().map(|v| v.join(" "));

        let selected = if self.all {
            names
        } else {
            let choice = match Picker::new()
                .items(&names)
                .filter(query.as_deref())
                .prompt("> ")
                .select()?
            {
                Some(s) => s,
                None => return Ok(()),
            };

            vec![choice]
        };

        for sel in selected {
            config.mux.kill_session(&sel)?;
            println!("Killed {}", &sel);
        }

        Ok(())
    }
}
