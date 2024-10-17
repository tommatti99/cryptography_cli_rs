use clap::{Parser, Subcommand};
use crate::algorithms::cesar_cipher::CesarCipherAlg;


#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CriptographyCliArgs {
    #[clap(subcommand)]
    pub algorithym: Algorithms
}


#[derive(Debug, Clone, Subcommand)]
pub enum Algorithms {
    /// Operation using Cesar Cipher
    CesarCipher(CesarCipherAlg)
}
