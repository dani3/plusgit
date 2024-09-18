use clap::{Arg, Command};
use std::process::ExitCode;

mod error;
mod plusgit;

pub const PLUSGIT_DIR: &'static str = ".plusgit";
pub const OBJECTS_DIR: &'static str = "objects";

fn cli() -> Command {
    Command::new("git")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(
            Command::new("init")
                .about("Create an empty Git repository")
                .arg(Arg::new("path").default_value(".")),
        )
        .subcommand(
            Command::new("hash-object")
                .about("Compute object ID and optionally create an object from a file")
                .arg(Arg::new("file").required(true)),
        )
}

fn main() -> ExitCode {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            if let Err(e) = plusgit::init(sub_matches.get_one::<String>("path").unwrap()) {
                println!("{e}");
                return ExitCode::from(1);
            }
        }
        Some(("hash-object", sub_matches)) => {
            if let Err(e) = plusgit::hash_object(sub_matches.get_one::<String>("file").unwrap()) {
                println!("{e}");
                return ExitCode::from(1);
            }
        }
        _ => unreachable!(),
    }

    ExitCode::from(0)
}
