use clap::{Parser, Subcommand};
use crate::algorithms::{advanced_encryption_standard::AdvancedEncryptionStandardAlg, cesar_cipher::CesarCipherAlg, transposition_cipher::TranspositionCipherAlg, vigenere_cipher::VigenereCipherAlg, xor_cipher::XorCipherAlg, data_encryption_standard::DataEncryptionStandardAlg};


#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CriptographyCliArgs {
    #[clap(subcommand)]
    pub algorithym: Algorithms
}


#[derive(Debug, Clone, Subcommand)]
pub enum Algorithms {
    /// Operation using Cesar Cipher
    CesarCipher(CesarCipherAlg),
    /// Operation using Vigenere Cipher
    VigenereCipher(VigenereCipherAlg),
    /// Operation using Xor Cipher
    XorCipher(XorCipherAlg),
    /// Operation using Transposition Cipher
    TranspositionCipher(TranspositionCipherAlg),
    /// Operation using Data Encryption Standard
    DataEncryptionStandard(DataEncryptionStandardAlg),
    /// Operation using AES
    AdvancedEncryptionStandard(AdvancedEncryptionStandardAlg)
}
