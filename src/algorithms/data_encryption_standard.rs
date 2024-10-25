use clap::Args;
use crate::{ops::{bits_into_string, make_64bits_blocks, string_into_bits}, Operations, 
            tables::{DES_INITIAL_PERMUTATION_TABLE, DES_FINAL_PERMUTATION_TABLE}};


#[derive(Debug, Args, Clone)]
pub struct DataEncryptionStandardAlg {
    /// Encode or Decode Operation
    #[arg(short, long)]
    pub operation: Operations,
    /// The secret word for encrypt
    #[arg(short, long)]
    pub key: String,
    /// The message to encode or decode
    #[arg(short, long)]
    pub message: String
}
impl DataEncryptionStandardAlg {
    
    pub fn execute(self) -> () {
        
        match self.operation {
            Operations::Encrypt => {
                self.encrypt();
            }
            Operations::Decrypt => {
                self.decrypt();
            }
        }
        
    }
    fn encrypt(self) {
        let blocks: Vec<[[u8; 8]; 8]> = make_64bits_blocks(string_into_bits(self.message));
        
        let permuted_blocks: Vec<[[u8; 8]; 8]> = blocks.iter().map(|block: &[[u8; 8]; 8]| initial_permutation(block.clone())).collect();




        let unpermuted_blocks: Vec<[[u8; 8]; 8]> = permuted_blocks.iter().map(|block: &[[u8; 8]; 8]| final_permutation(block.clone())).collect();

        println!("{}", bits_into_string(unpermuted_blocks.concat().concat()));
        }
    
    fn decrypt(self) {}



    
/* 
    fn key_dependent_computation() -> Vec<Vec<u8>> {}

    fn cipher_function_des() -> {}

    fn key_schedule(self) -> {}
    
    fn final_permutation() -> Vec<Vec<u8>> {}
*/
}

fn initial_permutation(block: [[u8; 8]; 8]) -> [[u8; 8]; 8] {
    let concat_block: Vec<u8> = block.concat();
    let permuted_block: Vec<u8> = DES_INITIAL_PERMUTATION_TABLE
            .concat()
            .iter()
            .map(|pos:&u8 | 
                {
                    concat_block[pos.clone() as usize - 1]
                })
            .collect();

    return make_64bits_blocks(permuted_block)[0];
}

fn final_permutation(block: [[u8; 8]; 8]) -> [[u8; 8]; 8] {
    let concat_block: Vec<u8> = block.concat();
    let unpermuted_block: Vec<u8> = DES_FINAL_PERMUTATION_TABLE
            .concat()
            .iter()
            .map(|pos:&u8 | 
                {
                    concat_block[pos.clone() as usize - 1]
                })
            .collect();

    return make_64bits_blocks(unpermuted_block)[0];
}


#[cfg(test)]
mod data_encryption_standard_test {
    use super::*;

    #[test]
    fn test_initial_permutation() -> () {
        assert_eq!(initial_permutation([[0; 8];8]), [[0; 8];8]);
        assert_eq!(initial_permutation([[1; 8];8]), [[1; 8];8]);

        let vec_test = make_64bits_blocks((1..=64).collect())[0];
        assert_eq!(initial_permutation(vec_test), DES_INITIAL_PERMUTATION_TABLE);
    }
    
    #[test]
    fn test_final_permutation() -> () {
        assert_eq!(final_permutation([[0; 8];8]), [[0; 8];8]);
        assert_eq!(final_permutation([[1; 8];8]), [[1; 8];8]);

        let vec_test = make_64bits_blocks((1..=64).collect())[0];
        assert_eq!(final_permutation(vec_test), DES_FINAL_PERMUTATION_TABLE);
    }
}