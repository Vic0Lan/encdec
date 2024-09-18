[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)

# Encdec
Simple CLI file encrypter written in Rust

## Getting Started

These instructions will give you a copy of the project up and running on
your local machine for development and testing purposes. See deployment
for notes on deploying the project on a live system.

### Prerequisites

Requirements for encdec:
- [Rust](https://www.rust-lang.org/tools/install)



### Installing
Short way:

    cargo install --git https://github.com/Vic0Lan/encdec.git
    
Long way:

    # Clone the repo
    $ git clone https://github.com/Vic0Lan/encdec.git

    # cd to the repo
    $ cd path/to/encdec

    # Build encdec
    $ cargo build --release
    $ cd target/release

    # Then move it somewhere in your $PATH. Here is an example:
    $ mv encdec ~/bin


### Usage
Note: The program needs to be in the same folder as nuo.nuo and yek.yek. 
Also, these two files need to have the same name in the example. 
You can download them and then change their content using keys that you decide.

Encrypt/Decrypt files or folders:    (encdec will dectect what you want to encrypt, no need for flags!)

    #No need to tell what this command does
    encdec --help

    #Encrypts/decrypts the file
    encdec encrypt /path/to/file
    encdec decrypt /path/to/file

    #Ecrypts/decrypts the file of the folder
    encdec encrypt /path/to/folder
    encdec decrypt /path/to/folder
    
Encrypts/decrypts a folder except a file

    encdec encrypt -x path/to/folder file/to/exclude
    encdec decrypt -x path/to/folder file/to/exclude


### Some upgrades to be done
Encrypts/decrypts a folder except a file type

    encdec encrypt path/to/folder -xt .file_type
    encdec decrypt path/to/folder -xt .file_type


[![forthebadge](https://forthebadge.com/images/featured/featured-built-with-love.svg)](https://forthebadge.com)
