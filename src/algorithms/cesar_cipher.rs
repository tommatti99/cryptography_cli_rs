use clap::{Args, ValueEnum};

#[derive(Debug, Args, Clone)]
pub struct CesarCipherAlg {
    /// Encode or Decode Operation
    #[arg(short, long)]
    pub operation: Operations,
    /// Shift number
    #[arg(short, long, default_value_t = 1)]
    pub shift: i8,
    /// The phrase to encode or decode
    #[arg(short, long)]
    pub phrase: String
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Operations {
    Encode,
    Decode
}

impl CesarCipherAlg {
    pub fn encode(self) -> () {
        println!("{:?}{}{}", self.operation, self.phrase, self.shift);
    }
}