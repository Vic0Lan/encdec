use chacha20poly1305::{aead::Aead, ChaCha20Poly1305, KeyInit};
use std::error::Error;
use std::path::PathBuf;
use std::{
    fs::{self, File},
    io::Error as ioError,
    io::Read,
};
use std::{process, thread};

#[derive(Clone, Copy)]
pub enum Operation {
    Encrypt,
    Decrypt,
}

pub fn start(path: &PathBuf, op: Operation, path_to_exclude: Option<PathBuf>) {
    if path.is_file() {
        read_and_do(path.to_path_buf(), op);
        return;
    }

    let mut handlers = Vec::new();

    for entry in fs::read_dir(path).unwrap_or_else(|err| {
        eprintln!("Error happened while reading directory: {}", err);
        process::exit(1);
    }) {
        match entry {
            Ok(entry) => {
                if let Some(ref file_path) = path_to_exclude {
                    let file_name = file_path.file_name().unwrap_or_else(|| {
                        eprintln!("Unknown error occurred, i fucked up");
                        process::exit(1);
                    });

                    if entry
                        .file_name()
                        .to_str()
                        .expect("Conversion failed")
                        .eq_ignore_ascii_case(file_name.to_str().expect("Conversion failed"))
                    {
                        println!("Skipped file: {}", file_name.to_string_lossy());
                        continue;
                    } /*else {
                          println!(
                              "The file names are {} {}",
                              entry.file_name().to_string_lossy(),
                              file_name.to_string_lossy()
                          );
                      }*/
                } /*else {
                      println!("Something wrong i can feel it");
                  }*/

                if !entry.path().is_dir() {
                    let thread = thread::spawn(move || {
                        read_and_do(entry.path(), op);
                    });

                    handlers.push(thread);
                }
            }
            Err(err) => eprintln!("Error occurred: {err}"),
        }
    }

    for thread in handlers {
        thread.join().unwrap_or_else(|err| {
            eprintln!("Thread had a problem: {:?}", err);
        })
    }
}

fn read_and_do(file_path: PathBuf, op: Operation) {
    let file = File::open(&file_path).unwrap_or_else(|err| {
        eprintln!(
            r"
            Error happend while opening file: {:?}
            Info: {err}
            ",
            file_path.file_name().unwrap(),
        );
        process::exit(1);
    });

    let bytes = file.bytes().map(|x| x.unwrap()).collect::<Vec<u8>>();

    let key = read_yek().expect("Failed to read yek");
    let nonce = read_nuo().expect("Failed to read nuo");

    let cypher = ChaCha20Poly1305::new(key.as_bytes().into());

    match op {
        Operation::Encrypt => {
            let encrypted_buf = encrypt(nonce.as_bytes(), &cypher, bytes).unwrap_or_else(|err| {
                eprintln!("Error happened during encryption {}", err);
                process::exit(1);
            });
            fs::write(file_path.as_path(), encrypted_buf).expect("Failed to write file");
            println!(
                "Successfully encrypted file: {:?}",
                file_path.file_name().expect("Failed to get file name")
            );
        }
        Operation::Decrypt => {
            let decrypted_buf = decrypt(nonce.as_bytes(), &cypher, bytes).unwrap_or_else(|err| {
                eprintln!("Error happened during decryption {}", err);
                process::exit(1);
            });
            fs::write(file_path.as_path(), decrypted_buf).expect("Failed to write file");
            println!(
                "Successfully decrypted file: {:?}",
                file_path.file_name().expect("Failed to get file name")
            );
        }
    }
}

fn encrypt(
    nonce: &[u8],
    cypher: &ChaCha20Poly1305,
    bytes: Vec<u8>,
) -> Result<Vec<u8>, Box<dyn Error>> {
    //   let cypher = ChaCha20Poly1305::new(key.into());
    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(&bytes);

    let buf = cypher.encrypt(nonce.into(), buf.as_ref()).unwrap();

    Ok(buf)
}

fn decrypt(
    nonce: &[u8],
    cypher: &ChaCha20Poly1305,
    bytes: Vec<u8>,
) -> Result<Vec<u8>, Box<dyn Error>> {
    // let cypher = ChaCha20Poly1305::new(key.into());
    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(&bytes);

    let buf = cypher.decrypt(nonce.into(), buf.as_ref()).unwrap();
    Ok(buf)
}

pub fn read_yek() -> Result<String, ioError> {
    let mut file = File::open("yek.yek")?;

    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    buf.truncate(buf.len() - 1);
    Ok(buf)
}

pub fn read_nuo() -> Result<String, ioError> {
    let mut file = File::open("nuo.nuo")?;

    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    buf.truncate(buf.len() - 1);
    Ok(buf)
}
