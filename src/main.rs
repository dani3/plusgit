use clap::{Arg, Command};
use object::ObjectKind;
use std::{process::ExitCode, str::FromStr};

mod error;
mod object;
mod plusgit;
mod repo;

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
                .about("Compute object ID and create an object from a file")
                .arg(Arg::new("file").required(true))
                .arg(
                    Arg::new("type")
                        .help("Specify the type of object to be created (default: 'blob'). Possible values are 'commit', 'tree', 'blob', and 'tag'.")
                        .short('t')
                        .long("type")
                        .required(false)
                        .default_value("blob"),
                ),
        )
        .subcommand(
            Command::new("cat-file")
                .about("Provide contents or details of repository objects")
                .arg(Arg::new("object").required(true)),
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
            if let Err(e) = plusgit::hash_object(
                sub_matches.get_one::<String>("file").unwrap(),
                ObjectKind::from_str(sub_matches.get_one::<String>("type").unwrap()).unwrap(),
            ) {
                println!("{e}");
                return ExitCode::from(1);
            }
        }
        // TODO: Handle different types.
        Some(("cat-file", sub_matches)) => {
            if let Err(e) = plusgit::cat_file(
                sub_matches.get_one::<String>("object").unwrap(),
                ObjectKind::Blob,
            ) {
                println!("{e}");
                return ExitCode::from(1);
            }
        }
        _ => unreachable!(),
    }

    ExitCode::from(0)
}
