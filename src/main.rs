use cli::parse_arguments;
use encdec::start;
mod cli;

//TODO:
//encdec
//CLI for the sake of the programmer (not so much)
//Read key and nonce from file: .dat would be nice, (screw it, created new file extension)✅
//Take folder name then encrypt every file (optional add a format or file to be excluded)

fn main() {
    println!("Testing chacha");

    //test_encrypt("Z/IMG_2918.JPG".as_ref());

    // start(&PathBuf::from_str("Z").unwrap(), Operation::Decrypt);

    let (path, op) = parse_arguments();
    start(&path, op);
}
