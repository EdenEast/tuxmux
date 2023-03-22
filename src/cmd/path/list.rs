use crate::{
    cli::PathList,
    cmd::ExecuteableCmd,
    data::{Location, Settings},
};

impl ExecuteableCmd for PathList {
    fn execute(self) -> eyre::Result<()> {
        let settings = match (self.global, self.local) {
            (true, false) => Settings::from_location(Location::Global)?,
            (false, true) => Settings::from_location(Location::Local)?,
            _ => Settings::new()?,
        };

        if self.single {
            settings.single_paths.iter().for_each(|s| println!("{}", s));
        } else if self.workspace {
            settings
                .workspace_paths
                .iter()
                .for_each(|s| println!("{}", s));
        } else {
            settings
                .single_paths
                .iter()
                .for_each(|s| println!("s {}", s));

            settings
                .workspace_paths
                .iter()
                .for_each(|s| println!("w {}", s));
        }

        Ok(())
    }
}
