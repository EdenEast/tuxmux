use std::path::Path;

use miette::miette;

use crate::{cmd::cli::Wcmd, config::Config, util::intersperse};

use super::Run;

impl Run for Wcmd {
    fn run(self) -> miette::Result<()> {
        let config = Config::load()?;
        let mux = config.mux;
        let name = Path::new(&self.window)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();

        let session_name = mux
            .session_name()
            .ok_or(miette!("failed to find current current session name"))?;

        let target = format!("{}:{}", session_name.trim(), name);

        if !mux.session_exists(&target) {
            mux.create_window(name)?;
        }

        let cmd: String = intersperse(self.cmds.iter().map(|f| f.as_str()), " ").collect();
        mux.send_command(&target, &cmd)?;

        Ok(())
    }
}
