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
Encrypt/Decrypt rust projects folder:

    encdec --help
    encdec encrypt /path/to/folder
    encdec decrypt /path/to/folder
    
[![forthebadge](https://forthebadge.com/images/featured/featured-built-with-love.svg)](https://forthebadge.com)
