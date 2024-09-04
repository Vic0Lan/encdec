use chacha20poly1305::{aead::Aead, ChaCha20Poly1305, KeyInit};
use std::error::Error;
use std::path::PathBuf;
use std::thread;
use std::{
    fs::{self, File},
    io::Error as ioError,
    io::Read,
};

#[derive(Clone, Copy)]
pub enum Operation {
    Encrypt,
    Decrypt,
}

pub fn start(path: &PathBuf, op: Operation) {
    if !path.is_dir() {
        println!("Sorry but i need a folder path");
        return;
    }

    match read_yek() {
        Ok(_) => (),
        Err(err) => eprintln!("Error while reading yek: {}", err),
    }

    match read_nuo() {
        Ok(_) => (),
        Err(err) => eprintln!("Error while reading nuo: {}", err),
    }

    let mut handlers = Vec::new();

    for entry in fs::read_dir(path).unwrap() {
        match entry {
            Ok(entry) => {
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
        thread.join().unwrap()
    }
}

fn read_and_do(file_path: PathBuf, op: Operation) {
    let file = File::open(&file_path).unwrap();

    let bytes = file.bytes().map(|x| x.unwrap()).collect::<Vec<u8>>();

    // let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    let key = read_yek().unwrap();
    //let cypher = ChaCha20Poly1305::new(key.into());
    let nonce = read_nuo().unwrap();

    let cypher = ChaCha20Poly1305::new(key.as_bytes().into());

    match op {
        Operation::Encrypt => {
            let encrypted_buf = encrypt(nonce.as_bytes(), &cypher, bytes).unwrap();
            fs::write(file_path, encrypted_buf).unwrap();
        }
        Operation::Decrypt => {
            let decrypted_buf = decrypt(nonce.as_bytes(), &cypher, bytes).unwrap();
            fs::write(file_path, decrypted_buf).unwrap();
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

    println!("Successfully encrypted");

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
    println!("Successfully decrypted");
    Ok(buf)
}

pub fn read_yek() -> Result<String, ioError> {
    let mut file = File::open("yek.yek")?;

    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    buf.truncate(buf.len() - 1);
    dbg!(buf.as_str());
    Ok(buf)
}

pub fn read_nuo() -> Result<String, ioError> {
    let mut file = File::open("nuo.nuo")?;

    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    buf.truncate(buf.len() - 1);
    dbg!(buf.as_str());
    Ok(buf)
}
