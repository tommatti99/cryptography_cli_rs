pub mod args;
pub mod alphabet;
pub mod algorithms {
    pub mod advanced_encryption_standard;
    pub mod cesar_cipher;
    pub mod data_encryption_standard;
    pub mod transposition_cipher;
    pub mod vigenere_cipher;
    pub mod xor_cipher;
}

use algorithms::{advanced_encryption_standard::AdvancedEncryptionStandardAlg, cesar_cipher::CesarCipherAlg, data_encryption_standard::DataEncryptionStandardAlg, transposition_cipher::TranspositionCipherAlg, vigenere_cipher::VigenereCipherAlg, xor_cipher::XorCipherAlg};
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
        Algorithms::CesarCipher(CesarCipherAlg {operation, shift, message}) => {
            let cesar_cipher_instance = CesarCipherAlg {operation, shift, message};
            cesar_cipher_instance.execute();
        }

        Algorithms::VigenereCipher(VigenereCipherAlg {operation, key, message}) => {
            let vigenere_cipher_instance = VigenereCipherAlg {operation, key, message};
            vigenere_cipher_instance.execute();
        }

        Algorithms::XorCipher(XorCipherAlg {operation, key, message}) => {
            let xor_cipher_instance = XorCipherAlg {operation, key, message};
            xor_cipher_instance.execute();
        }

        Algorithms::TranspositionCipher(TranspositionCipherAlg{}) => {
            let transposition_cipher_instance = TranspositionCipherAlg {};
            transposition_cipher_instance.execute();
        }
        
        Algorithms::DataEncryptionStandard(DataEncryptionStandardAlg {}) => {
            let data_encryption_standard_instance = DataEncryptionStandardAlg {};
            data_encryption_standard_instance.execute();
        }

        Algorithms::AdvancedEncryptionStandard(AdvancedEncryptionStandardAlg {}) => {
            let advanced_encryption_standard_instance = AdvancedEncryptionStandardAlg {};
            advanced_encryption_standard_instance.execute();
        }
    }
}
