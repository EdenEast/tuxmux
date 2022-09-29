use std::env;

use crate::data::{Location, Settings};
use clap::{builder::PossibleValuesParser, Arg, ArgMatches, Command};
use eyre::Result;

const CONFIG_OPTIONS: [&'static str; 2] = ["depth", "height"];

pub fn make_subcommand() -> Command {
    Command::new("config")
        .about("Get or set configuration options")
        .alias("c")
        .disable_version_flag(true)
        .args(&[
            Arg::new("name")
                .help("Name of configuration option")
                .value_parser(PossibleValuesParser::new(CONFIG_OPTIONS))
                .hide_possible_values(true)
                .required(false),
            Arg::new("value")
                .help("Value of the configuration option defined by name")
                .required(false),
            Arg::new("global")
                .help("Save to global $XDG_CONFIG_HOME instead of $XDG_DATA_HOME")
                .short('g')
                .long("global")
                .action(clap::ArgAction::SetTrue),
            Arg::new("edit")
                .help("Open config file in '$EDITOR'")
                .short('e')
                .long("edit")
                .action(clap::ArgAction::SetTrue),
            Arg::new("list")
                .help("List all config options and values")
                .short('l')
                .long("list")
                .action(clap::ArgAction::SetTrue),
        ])
}

pub fn execute(matches: &ArgMatches) -> Result<bool> {
    let location = if matches.get_flag("global") {
        Location::Global
    } else {
        Location::Local
    };

    if matches.get_flag("edit") {
        let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_owned());
        let file = Settings::filepath_from_location(location);
        std::process::Command::new(&editor).arg(file).status()?;

        return Ok(true);
    }

    if matches.get_flag("list") {
        let settings = if location == Location::Global {
            Settings::from_location(location)?
        } else {
            Settings::new()?
        };

        let content = toml::to_string_pretty(&settings)?;
        println!("{}", content);

        return Ok(true);
    }

    match (
        matches.get_one::<String>("name"),
        matches.get_one::<String>("value"),
    ) {
        (Some(name), Some(value)) => set_value(name, value, location)?,
        (Some(name), None) => get_value(name)?,
        _ => {}
    }

    Ok(true)
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
            settings.height = Some(value.parse()?);
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
