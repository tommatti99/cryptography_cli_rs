pub mod args;
pub mod alphabet;
pub mod algorithms {
    pub mod cesar_cipher;
    pub mod vigenere_cipher;
}

use algorithms::{cesar_cipher::CesarCipherAlg, vigenere_cipher::{self, VigenereCipherAlg}};
use args::{Algorithms, CriptographyCliArgs};
use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum Operations {
    Encode,
    Decode
}

fn main() {
    let args: CriptographyCliArgs = CriptographyCliArgs::parse();

    match args.algorithym {
        Algorithms::CesarCipher(CesarCipherAlg {operation, shift, phrase}) => {
            let cesar_cipher_instance = CesarCipherAlg {operation, shift, phrase};
            cesar_cipher_instance.execute();
        }

        Algorithms::VigenereCipher(VigenereCipherAlg {operation, key, phrase}) => {
            let vigenere_cipher_instance = VigenereCipherAlg {operation, key, phrase};
            vigenere_cipher_instance.execute();
        }
    }
}
