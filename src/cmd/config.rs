use std::env;

use crate::{
    cmd::cli::{Config, CONFIG_OPTIONS},
    data::{Location, Settings},
};

use eyre::Result;

use super::Run;

impl Run for Config {
    fn run(self) -> eyre::Result<()> {
        let location = if self.global {
            Location::Global
        } else {
            Location::Local
        };

        if self.edit {
            let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_owned());
            let file = Settings::filepath_from_location(location);
            std::process::Command::new(editor).arg(file).status()?;

            return Ok(());
        }

        if self.list {
            let settings = if location == Location::Global {
                Settings::from_location(location)?
            } else {
                Settings::new()?
            };

            let content = toml::to_string_pretty(&settings)?;
            println!("{}", content);

            return Ok(());
        }

        match (self.name, self.value) {
            (Some(name), Some(value)) => set_value(&name, &value, location)?,
            (Some(name), None) => get_value(&name)?,
            _ => {}
        }

        Ok(())
    }
}

fn get_value(name: &str) -> Result<()> {
    let settings = Settings::new()?;
    match name {
        "depth" => {
            if let Some(depth) = settings.depth {
                println!("{}", depth);
            }
        }
        "height" => {
            if let Some(height) = settings.height {
                println!("{}", height);
            }
        }
        "finder" => {
            if let Some(finder) = settings.finder {
                println!(
                    "{}",
                    match finder {
                        crate::finder::FinderChoice::Fzf => "fzf",
                        crate::finder::FinderChoice::Skim => "skim",
                    }
                );
            }
        }
        _ => {
            println!(
                "Unknown setting: '{}'. Possible values [{}]",
                name,
                CONFIG_OPTIONS.join(", ")
            );
        }
    };

    Ok(())
}

pub fn set_value(name: &str, value: &str, location: Location) -> Result<()> {
    let mut settings = Settings::from_location(location)?;

    match name {
        "depth" => {
            settings.depth = Some(value.parse()?);
        }
        "height" => {
            settings.height = Some(value.parse::<usize>()?.clamp(1, 100));
        }
        "finder" => {
            settings.finder = Some(value.parse()?);
        }
        _ => {
            println!(
                "Unknown setting: '{}'. Possible values [{}]",
                name,
                CONFIG_OPTIONS.join(", ")
            );
        }
    }

    settings.write(location)
}
