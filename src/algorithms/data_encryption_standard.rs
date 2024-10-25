use clap::Args;
use crate::{ops::{bits_into_string, make_64bits_blocks, string_into_bits}, Operations};

pub const INITIAL_PERMUTATION_TABLE: [[u8; 8]; 8] = [
                                    [58, 50, 42, 34, 26, 18, 10, 2], 
                                    [60, 52, 44, 36, 28, 20, 12, 4],
                                    [62, 54, 46, 38, 30, 22, 14, 6],
                                    [64, 56, 48, 40, 32, 24, 16, 8],
                                    [57, 49, 41, 33, 25, 17,  9, 1],
                                    [59, 51, 43, 35, 27, 19, 11, 3],
                                    [61, 53, 45, 37, 29, 21, 13, 5],
                                    [63, 55, 47, 39, 31, 23, 15, 7]
                                ];

pub const FINAL_PERMUTATION_TABLE: [[u8; 8]; 8] = [
                                    [40, 08, 48, 16, 56, 24, 64, 32], 
                                    [39, 07, 47, 15, 55, 23, 63, 31],
                                    [38, 06, 46, 14, 54, 22, 62, 30],
                                    [37, 05, 45, 13, 53, 21, 61, 29],
                                    [36, 04, 44, 12, 52, 20, 60, 28],
                                    [35, 03, 43, 11, 51, 19, 59, 27],
                                    [34, 02, 42, 10, 50, 18, 58, 26],
                                    [33, 01, 41, 09, 49, 17, 57, 25]
                                ];

pub const PC1: [[i8; 7]; 8] = [
                                    [57, 49, 41, 33, 25, 17, 09], 
                                    [01, 58, 50, 42, 34, 26, 18],
                                    [10, 02, 59, 51, 43, 35, 27],
                                    [19, 11, 03, 60, 52, 44, 36],
                                    [63, 55, 47, 39, 31, 23, 15],
                                    [07, 62, 54, 46, 38, 30, 22],
                                    [14, 06, 61, 53, 45, 37, 29],
                                    [21, 13, 05, 28, 20, 12, 04]
                                ];                                

pub const PC2: [[i8; 6]; 8] = [
                                    [14, 17, 11, 24, 01, 05], 
                                    [03, 28, 15, 06, 21, 10],
                                    [23, 19, 12, 04, 26, 08],
                                    [16, 07, 27, 20, 13, 02],
                                    [41, 52, 31, 37, 47, 55],
                                    [30, 40, 51, 45, 33, 48],
                                    [44, 49, 39, 56, 34, 53],
                                    [46, 42, 50, 36, 29, 32]
                                ];    

pub const EXPANSION: [[i8; 6]; 8] = [
                                    [32, 01, 02, 03, 04, 05], 
                                    [04, 05, 06, 07, 08, 09],
                                    [08, 09, 10, 11, 12, 13],
                                    [12, 13, 14, 15, 16, 17],
                                    [16, 17, 18, 19, 20, 21],
                                    [20, 21, 22, 23, 24, 25],
                                    [24, 25, 26, 27, 28, 29],
                                    [28, 29, 30, 31, 32, 01]
                                ];      

pub const PERMUTATION_TABLE: [[i8; 4]; 8] = [
                                    [16, 07, 20, 21], 
                                    [29, 12, 28, 17],
                                    [01, 15, 23, 26],
                                    [05, 18, 31, 10],
                                    [02, 08, 24, 14],
                                    [32, 27, 03, 09],
                                    [19, 13, 30, 06],
                                    [22, 11, 04, 25]
                                ];                                                                     
                                
pub const S_BOX_1_TABLE: [[i8; 16]; 4] = [
                                    [14, 04, 13, 01, 02, 15, 11, 08, 03, 10, 06, 12, 05, 09, 00, 07],
                                    [00, 15, 07, 04, 14, 02, 13, 01, 10, 06, 12, 11, 09, 05, 03, 08],
                                    [04, 01, 14, 08, 13, 06, 02, 11, 15, 12, 09, 07, 03, 10, 05, 00],
                                    [15, 12, 08, 02, 04, 09, 01, 07, 05, 11, 03, 14, 10, 00, 06, 13]
                                ];   

pub const S_BOX_2_TABLE: [[i8; 16]; 4] = [
                                    [15, 01, 08, 14, 06, 11, 03, 04, 09, 07, 02, 13, 12, 00, 05, 10],
                                    [03, 13, 04, 07, 15, 02, 08, 14, 12, 00, 01, 10, 06, 09, 11, 05],
                                    [00, 14, 07, 11, 10, 04, 13, 01, 05, 08, 12, 06, 09, 03, 02, 15],
                                    [13, 08, 10, 01, 03, 15, 04, 02, 11, 06, 07, 12, 00, 05, 14, 09]
                                ];   

pub const S_BOX_3_TABLE: [[i8; 16]; 4] = [
                                    [10, 00, 09, 14, 06, 03, 15, 05, 01, 13, 12, 07, 11, 04, 02, 08],
                                    [13, 07, 00, 09, 03, 04, 06, 10, 02, 08, 05, 14, 12, 11, 15, 01],
                                    [13, 06, 04, 09, 08, 15, 03, 00, 11, 01, 02, 12, 05, 10, 14, 07],
                                    [01, 10, 13, 00, 06, 09, 08, 07, 04, 15, 14, 03, 11, 05, 02, 12]
                                ];   

pub const S_BOX_4_TABLE: [[i8; 16]; 4] = [
                                    [07, 13, 14, 03, 00, 06, 09, 10, 01, 02, 08, 05, 11, 12, 04, 15],
                                    [13, 08, 11, 05, 06, 15, 00, 03, 04, 07, 02, 12, 01, 10, 14, 09],
                                    [10, 06, 09, 00, 12, 11, 07, 13, 15, 01, 03, 14, 05, 02, 08, 04],
                                    [03, 15, 00, 06, 10, 01, 13, 08, 09, 04, 05, 11, 12, 07, 02, 14]
                                ];   

pub const S_BOX_5_TABLE: [[i8; 16]; 4] = [
                                    [02, 12, 04, 01, 07, 10, 11, 06, 08, 05, 03, 15, 13, 00, 14, 09],
                                    [14, 11, 02, 12, 04, 07, 13, 01, 05, 00, 15, 10, 03, 09, 08, 06],
                                    [04, 02, 01, 11, 10, 13, 07, 08, 15, 09, 12, 05, 06, 03, 00, 14],
                                    [11, 08, 12, 07, 01, 14, 02, 13, 06, 15, 00, 09, 10, 04, 05, 03]
                                ];   

pub const S_BOX_6_TABLE: [[i8; 16]; 4] = [
    
                                    [12, 01, 10, 15, 09, 02, 06, 08, 00, 13, 03, 04, 14, 07, 05, 11],
                                    [10, 15, 04, 02, 07, 12, 09, 05, 06, 01, 13, 14, 00, 11, 03, 08],
                                    [09, 14, 15, 05, 02, 08, 12, 03, 07, 00, 04, 10, 01, 13, 11, 06],
                                    [04, 03, 02, 12, 09, 05, 15, 10, 11, 14, 01, 07, 06, 00, 08, 13]
                                ];   

pub const S_BOX_7_TABLE: [[i8; 16]; 4] = [
                                    [04, 11, 02, 14, 15, 00, 08, 13, 03, 12, 09, 07, 05, 10, 06, 01],
                                    [13, 00, 11, 07, 04, 09, 01, 10, 14, 03, 05, 12, 02, 15, 08, 06],
                                    [01, 04, 11, 13, 12, 03, 07, 14, 10, 15, 06, 08, 00, 05, 09, 02],
                                    [06, 11, 13, 08, 01, 04, 10, 07, 09, 05, 00, 15, 14, 02, 03, 12]
                                ];   

pub const S_BOX_8_TABLE: [[i8; 16]; 4] = [
                                    [13, 02, 08, 04, 06, 15, 11, 01, 10, 09, 03, 14, 05, 00, 12, 07],
                                    [01, 15, 13, 08, 10, 03, 07, 04, 12, 05, 06, 11, 00, 14, 09, 02],
                                    [07, 11, 04, 01, 09, 12, 14, 02, 00, 06, 10, 13, 15, 03, 05, 08],
                                    [02, 01, 14, 07, 04, 10, 08, 13, 15, 12, 09, 00, 03, 05, 06, 11]
                                ];   




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
    let permuted_block: Vec<u8> = INITIAL_PERMUTATION_TABLE
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
    let unpermuted_block: Vec<u8> = FINAL_PERMUTATION_TABLE
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
        assert_eq!(initial_permutation(vec_test), INITIAL_PERMUTATION_TABLE);
    }
    
    #[test]
    fn test_final_permutation() -> () {
        assert_eq!(final_permutation([[0; 8];8]), [[0; 8];8]);
        assert_eq!(final_permutation([[1; 8];8]), [[1; 8];8]);

        let vec_test = make_64bits_blocks((1..=64).collect())[0];
        assert_eq!(final_permutation(vec_test), FINAL_PERMUTATION_TABLE);
    }
}