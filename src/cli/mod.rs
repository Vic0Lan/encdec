use clap::{arg, command, value_parser, Arg, ArgAction, Command};
use encdec::Operation;
use std::{path::PathBuf, process};

pub fn parse_arguments() -> (PathBuf, Operation, Option<PathBuf>) {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .before_help("To work, encdec needs to be in the same folder with the nuo and yek files")
        .subcommand(
            Command::new("encrypt")
                .about("Encrypts a file or the files in a folder")
                .arg(arg!([PATH]).value_parser(value_parser!(PathBuf)))
                .arg(
                    Arg::new("exclude")
                        .short('x')
                        .action(ArgAction::SetTrue)
                        .help("flag used if you want to exclude a certain file or file type"),
                )
                .arg(
                    Arg::new("path-to-exclude")
                        .requires("exclude")
                        .required_if_eq("exclude", "true")
                        .num_args(1)
                        .value_parser(value_parser!(PathBuf))
                        .help("file o folder to exclude"),
                ),
        )
        .subcommand(
            Command::new("decrypt")
                .about("Decrypts a file or the files in a folder")
                .arg(arg!([PATH]).value_parser(value_parser!(PathBuf)))
                .arg(
                    Arg::new("exclude")
                        .short('x')
                        .action(ArgAction::SetTrue)
                        .help("flag used if you want to exclude a certain file or file type"),
                )
                .arg(
                    Arg::new("path-to-exclude")
                        .requires("exclude")
                        .required_if_eq("exclude", "true")
                        .num_args(1)
                        .value_parser(value_parser!(PathBuf))
                        .help("file o folder to exclude"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("encrypt", sub_matches)) => {
            let path = sub_matches.get_one::<PathBuf>("PATH");

            //let flag = sub_matches.get_flag("exclude");
            //println!("The flag exclude is {}", flag);

            if let Some(path_to_exclude) = sub_matches.get_one::<PathBuf>("path-to-exclude") {
                println!("path-to-exclude is {:?}", path_to_exclude);
                if path_to_exclude.is_dir() {
                    eprintln!("Need the path to the file you want to exclude");
                    process::exit(1);
                }
                (
                    path.unwrap().into(),
                    Operation::Encrypt,
                    Some(path_to_exclude.into()),
                )
            } else {
                (path.unwrap().into(), Operation::Encrypt, None)
            }
        }
        Some(("decrypt", sub_matches)) => {
            let path = sub_matches.get_one::<PathBuf>("PATH");

            // let flag = sub_matches.get_flag("exclude");
            // println!("The flag exclude is {}", flag);

            if let Some(path_to_exclude) = sub_matches.get_one::<PathBuf>("path-to-exclude") {
                println!("path-to-exclude is {:?}", path_to_exclude);
                if path_to_exclude.is_dir() {
                    eprintln!("Need the path to the file you want to exclude");
                    process::exit(1);
                }
                (
                    path.unwrap().into(),
                    Operation::Decrypt,
                    Some(path_to_exclude.into()),
                )
            } else {
                (path.unwrap().into(), Operation::Decrypt, None)
            }
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
