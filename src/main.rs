pub mod args;
pub mod algorithms {
    pub mod cesar_cipher;
}

use algorithms::cesar_cipher::CesarCipherAlg;
use args::{Algorithms, CriptographyCliArgs};
use clap::Parser;

fn main() {
    let args: CriptographyCliArgs = CriptographyCliArgs::parse();

    match args.algorithym {
        Algorithms::CesarCipher(CesarCipherAlg {operation, shift, phrase}) => {
            let cesar_cipher_instance = CesarCipherAlg {operation, shift, phrase};

            cesar_cipher_instance.encode();
        }
    }
}
