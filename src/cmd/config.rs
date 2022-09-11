use std::env;

use clap::{builder::PossibleValuesParser, Arg, ArgMatches, Command};
use eyre::Result;
use serde::__private::de::Content;
use tmgr::data::{Location, Settings};

include!(concat!(env!("OUT_DIR"), "/config-desc-map.rs"));

pub fn make_subcommand() -> Command<'static> {
    Command::new("config")
        .about("Get or set configuration options")
        .alias("c")
        .args(&[
            Arg::new("name")
                .help("Name of configuration option")
                .value_parser(PossibleValuesParser::new(CONFIG_DESCRIPTIONS.keys()))
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
            Arg::new("options")
                .help("List all config options with a description of each")
                .long("options")
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
        _ => {
            println!(
                "Unknown setting: '{}'. Possible values [{}]",
                name,
                CONFIG_DESCRIPTIONS
                    .keys()
                    .copied()
                    .collect::<Vec<_>>()
                    .join(", ")
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
        _ => {
            println!(
                "Unknown setting: '{}'. Possible values [{}]",
                name,
                CONFIG_DESCRIPTIONS
                    .keys()
                    .copied()
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }

    settings.write(location)
}
