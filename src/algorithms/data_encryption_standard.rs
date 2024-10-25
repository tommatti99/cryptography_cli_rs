use clap::Args;
use crate::{ops::{bits_into_string, byte_parity_verify, is_a_7bit_chunk, is_a_8bit_chunk, is_a_8bytes_block, left_shift_vec, make_64bits_blocks, permutation, string_into_bits}, tables::{DES_EXPANSION_TABLE, DES_FINAL_PERMUTATION_TABLE, DES_INITIAL_PERMUTATION_TABLE, DES_ITERATION_LEFT_SHIFT_TABLE, DES_PERMUTED_CHOICE_1, DES_PERMUTED_CHOICE_2}, Operations};

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
        // Prepare Keys
        let mut keys_for_16_rounds: Vec<PermutedChoice>;

        let key_in_bits: Vec<u8> = string_into_bits(self.key.clone());
        let key_in_blocks: Vec<&[u8]> = key_in_bits.chunks(8).collect();
        let verify_blocks_have_64bits: Vec<[u8; 8]> = key_in_blocks.iter().map(|chunk: &&[u8]| is_a_8bit_chunk(chunk)).collect();
        let verify_key_parity:  [[u8; 8]; 8] = is_a_8bytes_block(verify_blocks_have_64bits.iter().map(|chunk: &[u8; 8]| byte_parity_verify(*chunk)).collect());
        
        let c_0_d_0: PermutedChoice = PermutedChoice::permuted_choice_1(verify_key_parity);
        let mut left_key: Vec<u8> = c_0_d_0.permuted_choice_c.concat();
        let mut right_key: Vec<u8> = c_0_d_0.permuted_choice_d.concat();
        let mut shift_n: i32;

        for round in 0..=16 {
            shift_n = DES_ITERATION_LEFT_SHIFT_TABLE[round] as i32;
            
            left_key Vec<[u8;]>= left_shift_vec(left_key, shift_n as usize).chunks(7).collect();
            right_key = left_shift_vec(right_key, shift_n as usize).chunks(7).collect();

            keys_for_16_rounds.push(PermutedChoice {
                permuted_choice_c: left_key,  
                permuted_choice_d: right_key
            })
        }


        /////////////////////////////////




        let blocks: Vec<[[u8; 8]; 8]> = make_64bits_blocks(string_into_bits(self.clone().message));
        let permuted_blocks: Vec<Vec<u8>> = blocks.iter().map(|block: &[[u8; 8]; 8]| permutation(block.clone().concat(), DES_INITIAL_PERMUTATION_TABLE.concat())).collect();
        let perm_sep_blocks: Vec<BlockLeftRight> = permuted_blocks.iter().map(|block:&Vec<u8>| {BlockLeftRight::separate(block)}).collect();
        

        let output_blocks: Vec<Vec<u8>> = permuted_blocks.iter().map(|block: &Vec<u8>| permutation(block.clone(), DES_FINAL_PERMUTATION_TABLE.concat())).collect();
        println!("{}", bits_into_string(output_blocks.concat()));
        }
    
    fn decrypt(self) {}

    
}

struct BlockLeftRight {
    left: Vec<u8>,
    right: Vec<u8>
}
impl BlockLeftRight {
    fn separate(block: &[u8]) -> BlockLeftRight {
        let left: Vec<u8> = block[0..(block.len())/2].to_vec();
        let right: Vec<u8> = block[(block.len())/2..block.len()].to_vec();
        
        return 
            BlockLeftRight {
                left: left,
                right: right
            }
    }
}


#[derive(PartialEq, Debug)]
struct PermutedChoice {
    permuted_choice_c: [u8; 28],
    permuted_choice_d: [u8; 28]
}
impl PermutedChoice {
    fn permuted_choice_1(parity_verified_key: [[u8; 8]; 8]) -> PermutedChoice {
        let parity_verified_key_concat = parity_verified_key.concat();
        let permuted_key: Vec<u8> = permutation(parity_verified_key_concat, DES_PERMUTED_CHOICE_1.concat());

        let sep_key: BlockLeftRight = BlockLeftRight::separate(&permuted_key);
        let mut left_chunks: Vec<u8> = sep_key.left.chunks(28).map(|x| is_a_7bit_chunk(x)).collect();
        let mut right_chunks: Vec<u8> = sep_key.right.chunks(28).map(|x| is_a_7bit_chunk(x)).collect();


        return PermutedChoice {
            permuted_choice_c:  left_chunks.try_into().unwrap(),
            permuted_choice_d:  right_chunks.try_into().unwrap()
        }
    }
    
    fn permuted_choice_2(parity_verified_key: [[u8; 8]; 8]) -> PermutedChoice {
        let parity_verified_key_concat = parity_verified_key.concat();
        let permuted_key: Vec<u8> = permutation(parity_verified_key_concat, DES_PERMUTED_CHOICE_2.concat());

        let sep_key: BlockLeftRight = BlockLeftRight::separate(&permuted_key);
        let mut left_chunks: Vec<[u8; 7]> = sep_key.left.chunks(7).map(|x| is_a_7bit_chunk(x)).collect();
        let mut right_chunks: Vec<[u8; 7]> = sep_key.right.chunks(7).map(|x| is_a_7bit_chunk(x)).collect();

        while left_chunks.len() < 4 {
            left_chunks.push([0,0,0,0,0,0,0]);
        }

        while right_chunks.len() < 4 {
            right_chunks.push([0,0,0,0,0,0,0]);
        }

        return PermutedChoice {
            permuted_choice_c:  left_chunks.try_into().unwrap(),
            permuted_choice_d:  right_chunks.try_into().unwrap()
        }
    }
}

/*
fn cipher_function_des(left_right: BlockLeftRight, key: &[u8]) -> BlockLeftRight {}

fn ks_generate_key() -> () {}



fn key_derivation(key: String) -> [[u8; 8]; 8] {
    let key_57bits: &[u8] = &string_into_bits(key)[0..=57];

    
}
*/

#[cfg(test)]
mod data_encryption_standard_test {
    use super::*;

    #[test]
    fn test_permuted_choice() -> () {
        assert_eq!(PermutedChoice::permuted_choice_1([[0; 8];8]), PermutedChoice { permuted_choice_c: [[0; 7];4], permuted_choice_d: [[0; 7];4]});
        let vec_test: [[u8; 8]; 8] = make_64bits_blocks((1..=64).collect())[0];
        assert_eq!(PermutedChoice::permuted_choice_1(vec_test), PermutedChoice { 
            permuted_choice_c:                                     
            [[57, 49, 41, 33, 25, 17, 09], 
             [01, 58, 50, 42, 34, 26, 18],
             [10, 02, 59, 51, 43, 35, 27],
             [19, 11, 03, 60, 52, 44, 36]],

            permuted_choice_d: 
            [[63, 55, 47, 39, 31, 23, 15],
             [07, 62, 54, 46, 38, 30, 22],
             [14, 06, 61, 53, 45, 37, 29],
             [21, 13, 05, 28, 20, 12, 04]]
        });
    }
}