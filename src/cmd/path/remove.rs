use crate::{
    cli::PathRemove,
    cmd::ExecuteableCmd,
    data::{Location, PathKind, Settings},
    finder::FinderOptions,
};

impl ExecuteableCmd for PathRemove {
    fn execute(self) -> eyre::Result<()> {
        let location = if self.global {
            Location::Global
        } else {
            Location::Local
        };

        let mut settings = Settings::from_location(location)?;
        let iter: Vec<String> = settings
            .workspace_paths
            .iter()
            .cloned()
            .map(|mut s| {
                s.insert_str(0, "w| ");
                s
            })
            .chain(settings.single_paths.iter().cloned().map(|mut s| {
                s.insert_str(0, "s| ");
                s
            }))
            .collect();

        let opts = FinderOptions {
            multi: true,
            height: settings.height,
            ..Default::default()
        };

        let selected = settings.finder().execute(iter.iter(), opts)?;
        if selected.is_empty() {
            return Ok(());
        }

        for sel in selected {
            let (k, v) = sel.split_at(3);
            let kind = if k.starts_with('s') {
                PathKind::Single
            } else {
                PathKind::Workspace
            };
            match kind {
                PathKind::Single => settings.single_paths.remove(v),
                PathKind::Workspace => settings.workspace_paths.remove(v),
            };
        }

        settings.write(location)?;

        Ok(())
    }
}
