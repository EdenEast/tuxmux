use clap::ArgAction;
use clap::{value_parser, Arg, ArgMatches, Command};
use clap_complete::Generator;
use clap_complete::Shell;
use eyre::Result;

pub fn make_subcommand() -> Command {
    Command::new("completions")
        .about("Generate shell completions for your shell to stdout")
        .disable_version_flag(true)
        .arg(
            Arg::new("generator")
                .action(ArgAction::Set)
                .value_parser(value_parser!(Shell)),
        )
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    clap_complete::generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}

pub fn execute(matches: &ArgMatches) -> Result<()> {
    if let Some(shell) = matches.get_one::<Shell>("generator").copied() {
        let mut cmd = crate::cmd::make_clap_command();
        print_completions(shell, &mut cmd);
    }
    Ok(())
}
