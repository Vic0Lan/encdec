use cli::parse_arguments;
use encdec::start;
use encdec::{read_nuo, read_yek};
use std::process;

mod cli;

//TODO:
//encdec
//CLI for the sake of the programmer (not so much)
//Read key and nonce from file: .dat would be nice, (screw it, created new file extension)✅
//Take folder name then encrypt every file (optional add a format or file to be excluded)

fn main() {
    println!("Encdec output:");

    //test_encrypt("Z/IMG_2918.JPG".as_ref());

    let (path, op, path_to_exclude) = parse_arguments();

    if let Err(err) = read_yek() {
        eprintln!("Error while checking yek: {}", err);
        process::exit(1)
    }

    if let Err(err) = read_nuo() {
        eprintln!("Error while checking nuo: {}", err);
        process::exit(1)
    }

    start(&path, op, path_to_exclude);

    //Only for personal testing
    // match read_yek() {
    //     Ok(bytes) => {
    //         println!("{:?}", bytes.as_slice());
    //     }
    //     Err(err) => eprintln!("err in yek: {err}"),
    // }
    //
    // match read_nuo() {
    //     Ok(bytes) => {
    //         println!("{:?}", bytes.as_slice());
    //     }
    //     Err(err) => eprintln!("err in nuo {err}"),
    // }
    //
    // read_and_do("Z/file_example.mov".into(), rusty_tests::Operation::Decrypt)
}
