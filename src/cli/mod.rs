use clap::{arg, command, value_parser, Command};
use encdec::Operation;
use std::path::PathBuf;

pub fn parse_arguments() -> (PathBuf, Operation) {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .before_help("To work, encdec needs to be in the same folder with the nuo and yek files")
        .subcommand(
            Command::new("encrypt")
                .about("Encrypts a file or the files in a folder")
                .arg(arg!([PATH]).value_parser(value_parser!(PathBuf))),
        )
        .subcommand(
            Command::new("decrypt")
                .about("Decrypts a file or the files in a folder")
                .arg(arg!([PATH]).value_parser(value_parser!(PathBuf))),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("encrypt", sub_matches)) => {
            let path = sub_matches.get_one::<PathBuf>("PATH");
            println!("encdec encrypt was used, path is: {:?}", path);

            (path.unwrap().into(), Operation::Encrypt)
        }
        Some(("decrypt", sub_matches)) => {
            let path = sub_matches.get_one::<PathBuf>("PATH");
            println!("encdec decrypt was used, path is: {:?}", path);
            (path.unwrap().into(), Operation::Decrypt)
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
