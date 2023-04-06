use crate::{
    cmd::cli::PathList,
    cmd::Run,
    data::{Location, Settings},
};

impl Run for PathList {
    fn run(self) -> eyre::Result<()> {
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
