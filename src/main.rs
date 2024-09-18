use anyhow::Error;
use clap::{Arg, Command};

mod init;

pub const PLUSGIT_DIR: &'static str = ".plusgit";

fn cli() -> Command {
    Command::new("git")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(
            Command::new("init")
                .about("Create an empty Git repository")
                .arg(Arg::new("path").default_value(".")),
        )
}

fn main() -> Result<(), Error> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            init::run(sub_matches.get_one::<String>("path").unwrap());
        }
        _ => unreachable!(),
    }

    Ok(())
}
