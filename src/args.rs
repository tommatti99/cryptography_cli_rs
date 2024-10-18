use clap::{Parser, Subcommand};
use crate::algorithms::{cesar_cipher::CesarCipherAlg, vigenere_cipher::VigenereCipherAlg};


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
    VigenereCipher(VigenereCipherAlg)
}
