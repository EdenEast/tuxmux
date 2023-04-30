use std::sync::Arc;

use jwalk::WalkDir;

use crate::config::Config;

pub trait Walker {
    fn paths_from_walk(&self) -> Vec<String>;
}

impl Walker for Config {
    fn paths_from_walk(&self) -> Vec<String> {
        let mut result = Vec::new();

        let exclude_paths = Arc::new(self.exclude_path.clone());
        let definitions = Arc::new(self.definitions.clone());

        for workspace in &self.search.workspace {
            let exclude = exclude_paths.clone();
            let defs = definitions.clone();
            let walker = WalkDir::new(workspace)
                .skip_hidden(false)
                .max_depth(self.depth)
                .process_read_dir(move |_, _, _, children| {
                    // Exclude any children that match the exclude_path list
                    children.retain(|entry_result| {
                        entry_result
                            .as_ref()
                            .map(|entry| {
                                entry
                                    .path()
                                    .components()
                                    .last()
                                    .expect("always has last component")
                                    .as_os_str()
                                    .to_str()
                                    .map(|name| !exclude.iter().any(|e| *e == name))
                                    .unwrap_or(false)
                            })
                            .unwrap_or(false)
                    });
                });

            for entry in walker {
                match entry {
                    Ok(entry) => {
                        for (_, def) in defs.as_ref() {
                            if def.files.iter().any(|f| entry.path().join(f).exists()) {
                                result.push(entry.path().display().to_string());
                                break;
                            }
                        }
                    }
                    Err(_) => continue,
                }
            }
        }

        result
    }
}
