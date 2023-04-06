use crate::{
    cmd::cli::PathAdd,
    cmd::Run,
    data::{Location, PathKind, Settings},
};

impl Run for PathAdd {
    fn run(self) -> eyre::Result<()> {
        let cwd = std::env::current_dir()?;
        let paths = match &self.path {
            Some(vr) => vr.iter().map(|p| p.as_path()).collect(),
            None => vec![cwd.as_path()],
        };

        let location = if self.global {
            Location::Global
        } else {
            Location::Local
        };

        let kind = if self.workspace {
            PathKind::Workspace
        } else {
            PathKind::Single
        };

        let mut settings = Settings::from_location(location)?;
        for path in paths {
            let p = path.display().to_string();
            if settings.contains_path(&p) {
                println!("Path exists: {}", p);
            } else {
                settings.add_path(p, kind);
            }
        }

        settings.write(location)?;

        Ok(())
    }
}
