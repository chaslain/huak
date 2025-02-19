use super::utils::subcommand;
use clap::{arg, ArgAction, ArgMatches, Command};
use huak::{errors::CliResult, ops, project::Project};
use std::env;

/// Get the `fmt` subcommand.
pub fn cmd() -> Command<'static> {
    subcommand("fmt").about("Format Python code.").arg(
        arg!(--check)
            .id("check")
            .takes_value(false)
            .action(ArgAction::SetTrue)
            .help("Check if Python code is formatted."),
    )
}

/// Run the `fmt` command.
pub fn run(args: &ArgMatches) -> CliResult {
    // This command runs from the context of the cwd.
    let cwd = env::current_dir()?;
    let project = Project::from(cwd)?;
    let is_check = args.get_one::<bool>("check").unwrap();

    ops::fmt::fmt_project(&project, is_check)?;

    Ok(())
}
